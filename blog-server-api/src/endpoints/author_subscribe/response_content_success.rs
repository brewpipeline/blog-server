use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct AuthorSubscribeRequestContentSuccess;

impl ApiResponseContentBase for AuthorSubscribeRequestContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for AuthorSubscribeRequestContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "AUTHOR_SUBSCRIBE_OK"
    }

    fn description(&self) -> Option<String> {
        Some("author notification subscription state changed".to_string())
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
