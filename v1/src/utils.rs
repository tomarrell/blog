use std::error::Error;

use actix_web::{http, HttpResponse};

pub fn respond(tpl: Result<String, impl Error>, status: http::StatusCode) -> HttpResponse {
    match tpl {
        Ok(x) => respond_with_template(x.to_string(), status),
        Err(e) => respond_with_error(&e.to_string()),
    }
}

pub fn respond_with_error(err: &str) -> HttpResponse {
    HttpResponse::InternalServerError().body(format!(
        "Sorry, something went wrong. Please try again later.\n\nError: {}",
        err
    ))
}

pub fn respond_with_template(body: String, status: http::StatusCode) -> HttpResponse {
    HttpResponse::build(status)
        .content_type("text/html")
        .body(body)
}
