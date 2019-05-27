use std::cmp::{Ord, Ordering, PartialOrd};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TomlPost {
    pub title: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Post {
    pub doc_title: String,
    pub id: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub content: String,
}

impl Ord for Post {
    fn cmp(&self, other: &Post) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Post) -> Option<Ordering> {
        Some(self.date.cmp(&other.date))
    }
}
