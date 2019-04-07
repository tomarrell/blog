use std::error::Error;
use std::collections::BTreeMap;

use actix_web::{http, server, App, HttpRequest, HttpResponse, Responder};

mod templates;
use templates::Template;

fn handle_error(err: impl Error) -> HttpResponse {
    HttpResponse::InternalServerError().body(format!(
        "Sorry, something went wrong. Please try again later.\n\nError: {}",
        err.description()
    ))
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    let mut data = BTreeMap::new();
    data.insert("test", "cool");

    let tpl = req.state();
    let rendered = match tpl.tpl.layout(data) {
        Ok(x) => x,
        Err(e) => return handle_error(e),
    };

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

struct AppState {
    tpl: Template,
}

fn main() {
    server::new(move || {
        let mut templates = Template::new();
        templates.register_templates();

        App::with_state(AppState { tpl: templates })
            .resource("/", |r| r.method(http::Method::GET).f(index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
