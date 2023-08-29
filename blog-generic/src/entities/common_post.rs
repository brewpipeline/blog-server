use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonPost {
    pub title: String,
    pub published: u8,
    pub summary: String,
    pub content: Option<String>,
    pub tags: Vec<String>,
    pub image_url: Option<String>,
}
