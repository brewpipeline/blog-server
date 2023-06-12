use rbatis::rbdc::datetime::DateTime;
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
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
    pub registered_at: DateTime,
    pub status: Option<String>,
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user(&self, username: &String) -> DResult<Option<User>>;
    async fn create_user(&self, user: &User) -> DResult<()>;
}
