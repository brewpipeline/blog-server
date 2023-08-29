use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct DeletePostResponseContentSuccess;

impl ApiResponseContentBase for DeletePostResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for DeletePostResponseContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "DELETE_POST_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some("post record deleted".to_string())
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
