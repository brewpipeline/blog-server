use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub created_at: u64,
    pub content: Option<String>,
    pub author: Author,
    pub tags: Vec<Tag>,
}

impl Post {
    pub fn image_url(&self) -> String {
        format!(
            "https://source.unsplash.com/random/{}x{}?{}&sig={}",
            400,
            100,
            self.joined_tags_string("+"),
            self.slug,
        )
    }

    fn joined_tags_string(&self, sep: &str) -> String {
        self.tags
            .clone()
            .into_iter()
            .map(|v| v.title)
            .collect::<Vec<String>>()
            .join(sep)
    }

    pub fn tags_string(&self) -> String {
        self.joined_tags_string(", ")
    }
}
