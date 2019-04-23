use actix_web::middleware::Logger;
use actix_web::{fs, http, server, App, HttpRequest, HttpResponse, Responder};
use log::*;
use pretty_env_logger;

mod models;
mod parser;
mod templates;
mod utils;

const CONTENT_DIR: &'static str = "./posts";
const LOGGER_FORMAT: &'static str = r#"{ "ip": "%a", "host": "%{Host}i", "info": "%r", "status": "%s", "size": "%b", "referer": "%{Referer}i", "agent": "%{User-Agent}i", "timetaken": "%T" }"#;

struct AppState {
    tpl: templates::Template,
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    let posts = parser::parse_posts_dir(CONTENT_DIR);

    utils::respond(
        req.state().tpl.index(templates::IndexData { posts }),
        http::StatusCode::OK,
    )
}

fn post(req: &HttpRequest<AppState>) -> impl Responder {
    let post_name = match req.match_info().get("name") {
        Some(x) => x,
        None => {
            error!("No post name param supllied");
            return utils::respond_with_error("No post name supplied in path");
        }
    };

    info!("Fetching post directly: {}", post_name);

    // let posts = parser::parse_post(CONTENT_DIR);

    // let tpl = req.state();
    // let rendered = match tpl.tpl.layout(IndexData { posts }) {
    // Ok(x) => x,
    // Err(e) => return utils::handle_error(e),
    // };

    // HttpResponse::Ok().content_type("text/html").body(rendered)

    HttpResponse::Ok().body("A cool post")
}

fn not_found(req: &HttpRequest<AppState>) -> impl Responder {
    utils::respond(req.state().tpl.not_found(), http::StatusCode::NOT_FOUND)
}

fn main() {
    pretty_env_logger::init();

    let address = "127.0.0.1:8080";

    info!("Starting server on address {}", address);

    server::new(move || {
        let mut templates = templates::Template::new();
        templates.register_templates();

        App::with_state(AppState { tpl: templates })
            .middleware(Logger::new(LOGGER_FORMAT))
            .resource("/", |r| r.method(http::Method::GET).f(index))
            .resource("/post/{name}", |r| r.method(http::Method::GET).f(post))
            .handler("/public", fs::StaticFiles::new("./public").unwrap())
            .default_resource(|r| r.f(not_found))
    })
    .bind(address)
    .expect("Failed to start the server")
    .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[test]
    fn test_index() {
        let mut templates = templates::Template::new();
        templates.register_templates();

        let resp = test::TestRequest::with_state(AppState { tpl: templates })
            .run(&index)
            .unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK)
    }

    #[test]
    fn test_index_fails() {
        let templates = templates::Template::new();

        let resp = test::TestRequest::with_state(AppState { tpl: templates })
            .run(&index)
            .unwrap();

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[test]
    fn test_post() {
        let mut templates = templates::Template::new();
        templates.register_templates();

        let resp = test::TestRequest::with_state(AppState { tpl: templates })
            .run(&post)
            .unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK)
    }

    #[test]
    fn test_404() {
        let mut templates = templates::Template::new();
        templates.register_templates();

        let resp = test::TestRequest::with_state(AppState { tpl: templates })
            .run(&not_found)
            .unwrap();

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND)
    }
}
