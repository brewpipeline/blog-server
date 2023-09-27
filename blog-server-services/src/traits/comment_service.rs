use blog_generic::entities::CommonComment as ECommonComment;
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

use crate::utils::*;

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

impl From<(u64, ECommonComment)> for BaseComment {
    fn from(value: (u64, ECommonComment)) -> Self {
        BaseComment {
            post_id: value.1.post_id,
            author_id: value.0,
            created_at: time_utils::now_as_secs(),
            published: 1,
            content: value.1.content,
        }
    }
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
    async fn comment_by_id(&self, id: &u64) -> DResult<Option<Comment>>;
    async fn mark_deleted_by_id(&self, id: &u64) -> DResult<()>;
    async fn delete_by_post_id(&self, post_id: &u64) -> DResult<()>;
}
