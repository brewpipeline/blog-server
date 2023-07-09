use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tag {
    pub id: i64,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BasePost {
    pub author_id: i64,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub published: u8,
    pub created_at: i64,
    pub content: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Post {
    pub id: i64,
    pub author_slug: String,
    pub author_first_name: Option<String>,
    pub author_last_name: Option<String>,
    pub tags: Option<Vec<Tag>>,
    #[serde(flatten)]
    pub base: BasePost,
}

#[async_trait]
pub trait PostService: Send + Sync {
    async fn posts_count(&self) -> DResult<i64>;
    async fn posts(&self, offset: &i64, limit: &i64) -> DResult<Vec<Post>>;
    async fn post_by_id(&self, id: &i64) -> DResult<Option<Post>>;
    async fn post_by_slug(&self, slug: &String) -> DResult<Option<Post>>;
    async fn create_post(&self, post: &BasePost) -> DResult<i64>;
}
