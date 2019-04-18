use std::fs;

use log::error;
use log::info;
use serde::Serialize;
use toml::Value;

const CONTENT_DIR: &'static str = "./posts";

#[derive(Serialize)]
pub struct Post {
    pub name: String,
    pub date: String,
    pub description: String,
}

pub fn parse_posts() -> Vec<Post> {
    let dir = match fs::read_dir(CONTENT_DIR) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to open posts dir: {}", err);
            return vec![]
        },
    };

    let posts: Vec<Post> = dir.map(|x| {
        // let path = x.path();
        // info!("{:?}", value["foo"].as_str());

        Post{
            name: "This is a post title".to_string(),
            date: "2019-01-01 10:30:00Z".to_string(),
            description: "This is the description of a post. It covers a few interesting topics such as dogs, aeroplanes, and far off distant lands.".to_string(),
        }
    }).collect();

    posts
}
