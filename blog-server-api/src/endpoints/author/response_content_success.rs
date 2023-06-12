use blog_server_services::traits::user_service::User;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Author {
    authorname: String,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
    password_hash: String,
    registered_at: i64,
    status: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AuthorResponseContentSuccess {
    author: Author,
}

impl Into<AuthorResponseContentSuccess> for User {
    fn into(self) -> AuthorResponseContentSuccess {
        AuthorResponseContentSuccess {
            author: Author {
                authorname: self.username,
                first_name: self.first_name,
                middle_name: self.middle_name,
                last_name: self.last_name,
                mobile: self.mobile,
                email: self.email,
                password_hash: self.password_hash,
                registered_at: self.registered_at.unix_timestamp(),
                status: self.status,
            },
        }
    }
}

impl ApiResponseContentBase for AuthorResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for AuthorResponseContentSuccess {
    type Data = Self;

    fn identifier(&self) -> &'static str {
        "AUTHOR_FOUND"
    }

    fn description(&self) -> Option<String> {
        Some("author record found".to_string())
    }

    fn data(&self) -> &Self::Data {
        self
    }
}
