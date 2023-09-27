use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct CreateCommentContentSuccess;

impl ApiResponseContentBase for CreateCommentContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for CreateCommentContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "COMMENT_CREATED"
    }

    fn description(&self) -> Option<String> {
        Some(String::from("comment record created"))
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
