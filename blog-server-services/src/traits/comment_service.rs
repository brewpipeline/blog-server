use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseComment {
    pub post_id: u64,
    pub author_id: u64,
    pub created_at: u64,
    pub published: u8,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Comment {
    pub id: u64,
    #[serde(flatten)]
    pub base: BaseComment,
}

#[async_trait]
pub trait CommentService: Send + Sync {
    async fn comments_count_by_post_id(&self, post_id: &u64) -> DResult<u64>;
    async fn comments_by_post_id(
        &self,
        post_id: &u64,
        offset: &u64,
        limit: &u64,
    ) -> DResult<Vec<Comment>>;
    async fn create_comment(&self, post: &BaseComment) -> DResult<u64>;
}
