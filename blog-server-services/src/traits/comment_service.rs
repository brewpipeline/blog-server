use blog_generic::entities::{Comment as EComment, ShortAuthor as EShortAuthor};
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseComment {
    pub post_id: i64,
    pub author_id: i64,
    pub created_at: i64,
    pub published: u8,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Comment {
    pub id: i64,
    pub author_slug: String,
    pub author_first_name: Option<String>,
    pub author_last_name: Option<String>,
    #[serde(flatten)]
    pub base: BaseComment,
}

impl Into<EComment> for Comment {
    fn into(self) -> EComment {
        EComment {
            post_id: self.base.post_id,
            created_at: self.base.created_at,
            content: self.base.content,
            short_author: EShortAuthor {
                slug: self.author_slug,
                first_name: self.author_first_name,
                last_name: self.author_last_name,
            },
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
}
