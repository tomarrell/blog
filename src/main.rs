use std::path::{Path, PathBuf};

use actix_web::middleware::Logger;
use actix_web::{fs, http, server, App, HttpRequest, Responder, Result};
use comrak::{markdown_to_html, ComrakOptions};
use log::*;
use pretty_env_logger;

mod models;
mod parser;
mod templates;
mod utils;

const CONTENT_DIR: &'static str = "./posts";
const POST_FILE_EXT: &'static str = "toml";
const LOGGER_FORMAT: &'static str = r#"{ "ip": "%{X-PROXY-IP}i", "host": "%{Host}i", "info": "%r", "status": "%s", "size": "%b", "referer": "%{Referer}i", "agent": "%{User-Agent}i", "timetaken": "%T" }"#;

struct AppState {
    tpl: templates::Template,
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    let mut posts = parser::parse_posts_dir(Path::new(CONTENT_DIR));

    posts.reverse();

    utils::respond(
        req.state().tpl.index(templates::IndexData { posts }),
        http::StatusCode::OK,
    )
}

fn post(req: &HttpRequest<AppState>) -> impl Responder {
    let post_name = match req.match_info().get("name") {
        Some(x) => x,
        None => {
            error!("No post name param supplied");
            return utils::respond_with_error("No post name supplied in path");
        }
    };

    let path = Path::new(CONTENT_DIR)
        .join(post_name)
        .with_extension(POST_FILE_EXT);

    info!(
        "Fetching post with path: '{}'",
        path.to_str().unwrap_or("// Failed")
    );

    let raw_post = match parser::parse_post(&path) {
        Ok(post) => post,
        Err(_) => {
            error!(
                "Failed to find post at path: {}",
                path.to_str().unwrap_or("// Failed")
            );
            return utils::respond(req.state().tpl.not_found(), http::StatusCode::NOT_FOUND);
        }
    };

    let render_options = ComrakOptions {
        ext_table: true,
        ext_tasklist: true,
        ext_footnotes: true,
        ext_strikethrough: true,
        ..ComrakOptions::default()
    };

    let rendered_post = models::Post {
        content: markdown_to_html(&raw_post.content, &render_options),
        ..raw_post
    };

    utils::respond(req.state().tpl.post(rendered_post), http::StatusCode::OK)
}

fn not_found(req: &HttpRequest<AppState>) -> impl Responder {
    utils::respond(req.state().tpl.not_found(), http::StatusCode::NOT_FOUND)
}

fn file(path: PathBuf) -> impl Fn(&HttpRequest<AppState>) -> Result<fs::NamedFile> {
    move |_| Ok(fs::NamedFile::open(path.clone())?)
}

fn main() {
    pretty_env_logger::init_timed();

    let address = "127.0.0.1:8080";

    info!("Starting server on address {}", address);

    server::new(move || {
        let mut templates = templates::Template::new();
        templates.register_templates();

        App::with_state(AppState { tpl: templates })
            .middleware(Logger::new(LOGGER_FORMAT))
            .resource("/", |r| r.method(http::Method::GET).f(index))
            .resource("/post/{name}", |r| r.method(http::Method::GET).f(post))
            .resource("/favicon.ico", |r| {
                r.method(http::Method::GET)
                    .f(file(Path::new("./public/favicon.ico").to_path_buf()))
            })
            .resource("/sitemap.xml", |r| {
                r.method(http::Method::GET)
                    .f(file(Path::new("./sitemap.xml").to_path_buf()))
            })
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
            .param("name", "halle_leipzig")
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
