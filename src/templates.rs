use std::error::Error;

use handlebars::Handlebars;
use serde::Serialize;

use crate::models;

pub struct Template {
    hb: Handlebars,
}

#[derive(Serialize)]
pub struct IndexData {
    pub doc_title: String,
    pub posts: Vec<models::Post>,
}

impl Template {
    pub fn new() -> Template {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);

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
