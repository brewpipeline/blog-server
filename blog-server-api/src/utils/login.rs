use blog_server_services::traits::author_service::Author;
use chrono::{Months, Utc};
use jsonwebtoken::errors::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct LoginData {
    authorname: String,
    expire_timestamp: i64,
}

impl LoginData {
    pub fn with_authorname(authorname: String) -> Self {
        Self {
            authorname,
            expire_timestamp: (Utc::now() + Months::new(1)).timestamp(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expire_timestamp < Utc::now().timestamp()
    }
}

pub fn create_login_token(author: Author) -> Result<String> {
    super::jwt::encode(
        &LoginData::with_authorname(author.authorname),
        &author.password_hash,
    )
}

/*
pub fn validate_login_token(token: String, author_service: Arc<Box<dyn AuthorService>>) -> Option<Author> {
    super::jwt::decode(token, additional_secret)
}
*/
