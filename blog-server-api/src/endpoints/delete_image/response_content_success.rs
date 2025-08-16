use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct DeleteImageResponseContentSuccess;

impl ApiResponseContentBase for DeleteImageResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for DeleteImageResponseContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "DELETE_IMAGE_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some("image deleted".to_string())
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}

