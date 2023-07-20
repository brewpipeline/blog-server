use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseAuthor {
    pub slug: String,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub password_hash: String,
    pub registered_at: i64,
    pub status: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Author {
    pub id: u64,
    #[serde(flatten)]
    pub base: BaseAuthor,
}

#[async_trait]
pub trait AuthorService: Send + Sync {
    async fn authors_count_by_query(&self, query: &String) -> DResult<i64>;
    async fn authors_by_query(
        &self,
        query: &String,
        offset: &i64,
        limit: &i64,
    ) -> DResult<Vec<Author>>;
    async fn authors_count(&self) -> DResult<i64>;
    async fn authors(&self, offset: &i64, limit: &i64) -> DResult<Vec<Author>>;
    async fn author_by_id(&self, id: &u64) -> DResult<Option<Author>>;
    async fn author_by_slug(&self, slug: &String) -> DResult<Option<Author>>;
    async fn create_author(&self, author: &BaseAuthor) -> DResult<i64>;
}
