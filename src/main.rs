use std::collections::BTreeMap;
use std::error::Error;

use actix_web::middleware::Logger;
use actix_web::{fs, http, server, App, HttpRequest, HttpResponse, Responder};
use log::*;
use pretty_env_logger;

mod parser;
use parser::parse_posts;

mod templates;
use templates::Template;

struct AppState {
    tpl: Template,
}

fn handle_error(err: impl Error) -> HttpResponse {
    HttpResponse::InternalServerError().body(format!(
        "Sorry, something went wrong. Please try again later.\n\nError: {}",
        err.description()
    ))
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    let posts = parse_posts();
    let mut data = BTreeMap::new();
    data.insert("posts", posts);

    let tpl = req.state();
    let rendered = match tpl.tpl.layout(data) {
        Ok(x) => x,
        Err(e) => return handle_error(e),
    };

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

fn main() {
    pretty_env_logger::init();

    let address = "127.0.0.1:8080";

    info!("Starting server on address {}", address);

    server::new(move || {
        let mut templates = Template::new();
        templates.register_templates();

        App::with_state(AppState { tpl: templates })
            .middleware(Logger::new(r#"{ "ip": "%a", "host": "%{Host}i", "info": "%r", "status": "%s", "size": "%b", "referer": "%{Referer}i", "agent": "%{User-Agent}i", "timetaken": "%T" }"#))
            .resource("/", |r| r.method(http::Method::GET).f(index))
            .handler("/public", fs::StaticFiles::new("./public").unwrap())
    })
    .bind(address)
    .unwrap()
    .run();
}
