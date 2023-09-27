use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct DeleteCommentResponseContentSuccess;

impl ApiResponseContentBase for DeleteCommentResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for DeleteCommentResponseContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "DELETE_COMMENT_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some("comment record deleted".to_string())
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
