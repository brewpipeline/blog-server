use crate::entities::Author;
use blog_server_services::traits::author_service::Author as ServiceAuthor;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AuthorMeResponseContentSuccess {
    author: Author,
}

impl Into<AuthorMeResponseContentSuccess> for ServiceAuthor {
    fn into(self) -> AuthorMeResponseContentSuccess {
        AuthorMeResponseContentSuccess {
            author: self.into(),
        }
    }
}

impl ApiResponseContentBase for AuthorMeResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for AuthorMeResponseContentSuccess {
    type Data = Self;

    fn identifier(&self) -> &'static str {
        "AUTHOR_ME_OK"
    }

    fn description(&self) -> Option<String> {
        Some("auth passed and self author profile returned".to_string())
    }

    fn data(&self) -> &Self::Data {
        self
    }
}
