use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct PostPoolResponseContentSuccess;

impl ApiResponseContentBase for PostPoolResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for PostPoolResponseContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "POST_POOL_OK"
    }

    fn description(&self) -> Option<String> {
        Some("post pool state changed".to_string())
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
