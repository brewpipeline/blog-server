use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct AuthorBlockResponseContentSuccess;

impl ApiResponseContentBase for AuthorBlockResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for AuthorBlockResponseContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "AUTHOR_BLOCK_OK"
    }

    fn description(&self) -> Option<String> {
        Some("author block state changed".to_string())
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
