use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum PublishedType {
    Unpublished = 0,
    Published = 1,
    Hidden = 2,
}

impl PublishedType {
    pub fn is_published(&self) -> bool {
        match self {
            PublishedType::Unpublished => false,
            PublishedType::Published | PublishedType::Hidden => true,
        }
    }
}

impl From<u8> for PublishedType {
    fn from(value: u8) -> Self {
        match value {
            0 => PublishedType::Unpublished,
            1 => PublishedType::Published,
            2 => PublishedType::Hidden,
            _ => PublishedType::Unpublished,
        }
    }
}

impl From<PublishedType> for u8 {
    fn from(status: PublishedType) -> Self {
        status as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub published_type: PublishedType,
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
