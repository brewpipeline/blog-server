use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct AuthorOverrideSocialDataResponseContentSuccess;

impl Into<AuthorOverrideSocialDataResponseContentSuccess> for () {
    fn into(self) -> AuthorOverrideSocialDataResponseContentSuccess {
        AuthorOverrideSocialDataResponseContentSuccess
    }
}

impl ApiResponseContentBase for AuthorOverrideSocialDataResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for AuthorOverrideSocialDataResponseContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "AUTHOR_OVERRIDE_SOCIAL_DATA_OK"
    }

    fn description(&self) -> Option<String> {
        Some("author block state changed".to_string())
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
