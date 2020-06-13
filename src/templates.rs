use std::error::Error;

use chrono::DateTime;
use handlebars::{handlebars_helper, Handlebars};
use log::*;
use serde::Serialize;

use crate::models;

pub struct Template {
    hb: Handlebars,
}

#[derive(Serialize)]
pub struct IndexData {
    pub posts: Vec<models::Post>,
}

// Return the ordinal indicator for the day
// of the month given a DateLike.
fn day_ordinal_indicator<'a, T>(date: &'a T) -> &'a str
where
    T: chrono::Datelike,
{
    match date.day() {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    }
}

handlebars_helper!(fmt_date: |v: str| {
    let date = match DateTime::parse_from_rfc3339(v) {
        Ok(d) => {
            let ord = day_ordinal_indicator(&d);
            d.format(&format!("%e{} %B %Y", ord)).to_string()
        },
        Err(_) => {
            error!("failed to parse time from post");
            "N/A".to_string()
        },
    };

    date
});

impl Template {
    pub fn new() -> Template {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("fmt-date", Box::new(fmt_date));

        Template { hb: handlebars }
    }

    pub fn not_found(&self) -> Result<String, impl Error> {
        self.hb.render("404", &())
    }

    pub fn index(&self, data: IndexData) -> Result<String, impl Error> {
        self.hb.render("index", &data)
    }

    pub fn post(&self, data: models::Post) -> Result<String, impl Error> {
        self.hb.render("post", &data)
    }

    pub fn register_templates(&mut self) {
        let paths = vec![
            ("index", "templates/index.html"),
            ("post", "templates/post.html"),
            ("404", "templates/404.html"),
            // Consumed by other templates
            ("_base", "templates/base.html"),
            ("_heading", "templates/heading.html"),
            ("_snippet", "templates/snippet.html"),
        ];

        paths.iter().for_each(|x| {
            assert!(self.hb.register_template_file(x.0, x.1).is_ok());
        });
    }
}

#[cfg(test)]
mod tests {}
