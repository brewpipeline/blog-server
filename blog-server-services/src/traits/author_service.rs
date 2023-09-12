use std::collections::HashSet;

use blog_generic::entities::Author as EAuthor;
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
    pub registered_at: u64,
    pub status: Option<String>,
    pub image_url: Option<String>,
    pub editor: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Author {
    pub id: u64,
    #[serde(flatten)]
    pub base: BaseAuthor,
}

impl Into<EAuthor> for Author {
    fn into(self) -> EAuthor {
        EAuthor {
            id: self.id,
            slug: self.base.slug,
            first_name: self.base.first_name,
            last_name: self.base.last_name,
            middle_name: self.base.middle_name,
            mobile: self.base.mobile,
            email: self.base.email,
            registered_at: self.base.registered_at,
            status: self.base.status,
            image_url: self.base.image_url,
            editor: self.base.editor,
        }
    }
}

#[async_trait]
pub trait AuthorService: Send + Sync {
    async fn authors_count_by_query(&self, query: &String) -> DResult<u64>;
    async fn authors_by_query(
        &self,
        query: &String,
        offset: &u64,
        limit: &u64,
    ) -> DResult<Vec<Author>>;
    async fn authors_count(&self) -> DResult<u64>;
    async fn authors(&self, offset: &u64, limit: &u64) -> DResult<Vec<Author>>;
    async fn authors_by_ids(&self, ids: &HashSet<u64>) -> DResult<Vec<Author>>;
    async fn author_by_id(&self, id: &u64) -> DResult<Option<Author>>;
    async fn author_by_slug(&self, slug: &String) -> DResult<Option<Author>>;
    async fn create_author(&self, author: &BaseAuthor) -> DResult<u64>;
}
