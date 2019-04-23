use std::error::Error;
use std::fs;
use std::path::Path;

use log::error;

use crate::models::Post;

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

fn parse_post(path: &Path) -> Result<Post, Box<dyn Error>> {
    let post: String = fs::read_to_string(path)?;
    let parsed: Post = toml::from_str(&post)?;

    Ok(parsed)
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
                name: "TestName".to_string(),
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
                name: "TestName".to_string(),
                date: Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                description: "Test description".to_string(),
                content: "Test content".to_string(),
            }
        )
    }

    #[test]
    fn test_parsing_sorted() {
        let posts = parse_posts_dir("./tests/sort");
        let res: Vec<&str> = posts
            .iter()
            .map(|x| x.name.as_ref())
            .collect();

        assert_eq!(
            res,
            vec!["First", "Second"]
        )
    }
}
