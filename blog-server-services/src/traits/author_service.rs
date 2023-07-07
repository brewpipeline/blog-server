use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: Option<i64>,
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

#[async_trait]
pub trait AuthorService: Send + Sync {
    async fn authors_count(&self) -> DResult<i64>;
    async fn get_authors(&self, offset: &i64, limit: &i64) -> DResult<Vec<Author>>;
    async fn get_author_by_id(&self, id: &i64) -> DResult<Option<Author>>;
    async fn get_author_by_slug(&self, slug: &String) -> DResult<Option<Author>>;
    async fn create_author(&self, author: &Author) -> DResult<()>;
}
