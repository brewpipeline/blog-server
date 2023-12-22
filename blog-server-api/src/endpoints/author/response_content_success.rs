use blog_generic::entities::AuthorContainer;
use blog_server_services::traits::author_service::Author as ServiceAuthor;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct AuthorResponseContentSuccess {
    pub(crate) container: AuthorContainer,
}

impl Into<AuthorResponseContentSuccess> for ServiceAuthor {
    fn into(self) -> AuthorResponseContentSuccess {
        AuthorResponseContentSuccess {
            container: AuthorContainer {
                author: self.into(),
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
    type Data = AuthorContainer;

    fn identifier(&self) -> &'static str {
        "AUTHOR_FOUND"
    }

    fn description(&self) -> Option<String> {
        Some("author record found".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.container
    }
}
