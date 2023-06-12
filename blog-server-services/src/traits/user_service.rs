use rbatis::rbdc::datetime::DateTime;
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub password_hash: String,
    pub registered_at: DateTime,
    pub status: Option<String>,
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user(&self, username: &String) -> DResult<Option<User>>;
    async fn create_user(&self, user: &User) -> DResult<()>;
}
