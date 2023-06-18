use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub authorname: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    #[serde(rename = "registeredAt")]
    pub registered_at: i64,
    pub status: Option<String>,
}

#[async_trait]
pub trait AuthorService: Send + Sync {
    async fn get_author(&self, authorname: &String) -> DResult<Option<Author>>;
    async fn create_author(&self, author: &Author) -> DResult<()>;
}
