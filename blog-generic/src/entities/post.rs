use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum PublishType {
    Unpublished = 0,
    Published = 1,
    Hidden = 2,
}

impl PublishType {
    pub fn is_published(&self) -> bool {
        match self {
            PublishType::Unpublished => false,
            PublishType::Published | PublishType::Hidden => true,
        }
    }
}

impl From<u8> for PublishType {
    fn from(value: u8) -> Self {
        match value {
            0 => PublishType::Unpublished,
            1 => PublishType::Published,
            2 => PublishType::Hidden,
            _ => PublishType::Unpublished,
        }
    }
}

impl From<PublishType> for u8 {
    fn from(status: PublishType) -> Self {
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
    pub publish_type: PublishType,
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
