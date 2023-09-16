use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub published: u8,
    pub created_at: u64,
    pub content: Option<String>,
    pub author: Author,
    pub tags: Vec<Tag>,
    pub image_url: Option<String>,
}

impl Post {
    pub fn joined_tags_string(&self, sep: &str) -> String {
        self.tags
            .clone()
            .into_iter()
            .map(|v| v.title)
            .collect::<Vec<String>>()
            .join(sep)
    }
}
