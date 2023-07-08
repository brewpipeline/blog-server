use crate::entities::Post;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostsResponseContentSuccess {
    pub(super) posts: Vec<Post>,
    pub(super) total: i64,
    pub(super) offset: i64,
    pub(super) limit: i64,
}

impl ApiResponseContentBase for PostsResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for PostsResponseContentSuccess {
    type Data = Self;

    fn identifier(&self) -> &'static str {
        "POSTS_OK"
    }

    fn description(&self) -> Option<String> {
        Some("posts list returned".to_string())
    }

    fn data(&self) -> &Self::Data {
        self
    }
}
