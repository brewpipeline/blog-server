use blog_generic::entities::AuthorsContainer;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct AuthorsResponseContentSuccess {
    pub(super) container: AuthorsContainer,
}

impl Into<AuthorsResponseContentSuccess> for AuthorsContainer {
    fn into(self) -> AuthorsResponseContentSuccess {
        AuthorsResponseContentSuccess { container: self }
    }
}

impl ApiResponseContentBase for AuthorsResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for AuthorsResponseContentSuccess {
    type Data = AuthorsContainer;

    fn identifier(&self) -> &'static str {
        "AUTHORS_OK"
    }

    fn description(&self) -> Option<String> {
        Some("authors list returned".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.container
    }
}
