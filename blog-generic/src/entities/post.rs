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
    pub image_url: Option<String>,
}
