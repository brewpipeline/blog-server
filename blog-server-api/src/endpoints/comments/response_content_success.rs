use crate::entities::Comment;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommentsResponseContentSuccess {
    pub(super) comments: Vec<Comment>,
    pub(super) total: i64,
    pub(super) offset: i64,
    pub(super) limit: i64,
}

impl ApiResponseContentBase for CommentsResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for CommentsResponseContentSuccess {
    type Data = Self;

    fn identifier(&self) -> &'static str {
        "COMMENTS_OK"
    }

    fn description(&self) -> Option<String> {
        Some("comments list returned".to_string())
    }

    fn data(&self) -> &Self::Data {
        self
    }
}
