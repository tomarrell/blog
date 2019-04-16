use std::error::Error;
use serde::ser::Serialize;

use handlebars::Handlebars;

pub struct Template {
    hb: Handlebars,
}

impl Template {
    pub fn new() -> Template {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);

        Template {
            hb: handlebars,
        }
    }

    pub fn layout<F>(&self, data: F) -> Result<String, impl Error>
        where F: Serialize
    {
        self.hb.render("layout", &data)
    }

    pub fn register_templates(&mut self) {
        let paths = vec![
            ("layout", "templates/layout.html"),
            ("post", "templates/post.html"),
        ];

        paths.iter().for_each(|x| {
            assert!(self.hb.register_template_file(x.0, x.1).is_ok());
        });
    }
}
