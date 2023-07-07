use crate::entities::Author;
use blog_server_services::traits::author_service::Author as ServiceAuthor;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorResponseContentSuccess {
    author: Author,
}

impl Into<AuthorResponseContentSuccess> for ServiceAuthor {
    fn into(self) -> AuthorResponseContentSuccess {
        AuthorResponseContentSuccess {
            author: self.into(),
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
