use std::cmp::{Ord, PartialOrd, Ordering};

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Post {
    pub name: String,
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
