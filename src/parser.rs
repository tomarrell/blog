use std::error::Error; use std::fs;
use std::path::Path;

use log::error;

use crate::models::{Post, TomlPost};

pub fn parse_posts_dir(dir_str: &str) -> Vec<Post> {
    let dir = fs::read_dir(dir_str).expect("Failed to open posts directory");

    let mut posts: Vec<Post> = dir
        .map(|x| -> Result<Post, Box<dyn Error>> { parse_post(&x?.path()) })
        .map(|x| {
            if let Err(e) = x {
                error!("A post failed to parse: {}", e);
                return None;
            };

            x.ok()
        })
        .filter_map(|x| x)
        .collect();

    posts.sort();
    posts
}

pub fn parse_post(path: &Path) -> Result<Post, Box<dyn Error>> {
    let post: String = fs::read_to_string(path)?;
    let parsed: TomlPost = toml::from_str(&post)?;

    let stem = path
        .file_stem()
        .ok_or("Invalid stem on file path")?
        .to_str()
        .ok_or("Unable to convert file name to str")?
        .to_string();

    Ok(Post {
        id: stem,
        title: parsed.title,
        date: parsed.date,
        description: parsed.description,
        content: parsed.content,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;
    use chrono::Utc;

    #[test]
    fn test_parsing_dir() {
        let res = parse_posts_dir("./tests/basic");

        assert_eq!(
            res,
            vec![Post {
                id: "test".to_string(),
                title: "TestName".to_string(),
                date: Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                description: "Test description".to_string(),
                content: "Test content".to_string(),
            }]
        )
    }

    #[test]
    fn test_parsing_single() {
        let res = parse_post(Path::new("./tests/basic/test.toml"))
            .expect("Failed to parse individual Post");

        assert_eq!(
            res,
            Post {
                id: "test".to_string(),
                title: "TestName".to_string(),
                date: Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                description: "Test description".to_string(),
                content: "Test content".to_string(),
            }
        )
    }

    #[test]
    fn test_parsing_sorted() {
        let posts = parse_posts_dir("./tests/sort");
        let res: Vec<&str> = posts.iter().map(|x| x.title.as_ref()).collect();

        assert_eq!(res, vec!["First", "Second"])
    }
}
