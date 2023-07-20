use blog_server_services::traits::post_service::BasePost;
use serde::{Deserialize, Serialize};

use crate::utils::time_utils;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub published: u8,
    pub content: Option<String>,
    pub tags: Vec<ShortTag>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortTag {
    pub title: String,
}

impl CreatePost {
    pub fn into(self, author_id: u64) -> BasePost {
        BasePost {
            author_id,
            created_at: time_utils::now_as_secs(),
            title: self.title,
            slug: self.slug,
            summary: self.summary,
            published: self.published,
            content: self.content,
        }
    }
}
