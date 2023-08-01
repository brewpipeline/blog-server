use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub post_id: i64,
    pub created_at: i64,
    pub content: String,
    pub short_author: ShortAuthor,
}
