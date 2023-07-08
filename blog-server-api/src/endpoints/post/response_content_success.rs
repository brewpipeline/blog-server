use crate::entities::Post;
use blog_server_services::traits::post_service::Post as ServicePost;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostResponseContentSuccess {
    post: Post,
}

impl Into<PostResponseContentSuccess> for ServicePost {
    fn into(self) -> PostResponseContentSuccess {
        PostResponseContentSuccess { post: self.into() }
    }
}

impl ApiResponseContentBase for PostResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for PostResponseContentSuccess {
    type Data = Self;

    fn identifier(&self) -> &'static str {
        "POST_FOUND"
    }

    fn description(&self) -> Option<String> {
        Some("post record found".to_string())
    }

    fn data(&self) -> &Self::Data {
        self
    }
}
