use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct PostUpdateRecommendedResponseContentSuccess;

impl ApiResponseContentBase for PostUpdateRecommendedResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for PostUpdateRecommendedResponseContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "POST_UPDATE_RECOMMENDED_OK"
    }

    fn description(&self) -> Option<String> {
        Some("post recommended state changed".to_string())
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
