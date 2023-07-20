use crate::entities::Post;
use blog_server_services::traits::post_service::Post as ServicePost;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostContentSuccess {
    created_post: Post,
}

impl Into<CreatePostContentSuccess> for ServicePost {
    fn into(self) -> CreatePostContentSuccess {
        CreatePostContentSuccess {
            created_post: (self.into()),
        }
    }
}

impl ApiResponseContentBase for CreatePostContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for CreatePostContentSuccess {
    type Data = Self;

    fn identifier(&self) -> &'static str {
        "POST_CREATED"
    }

    fn description(&self) -> Option<String> {
        Some(String::from("post record created"))
    }

    fn data(&self) -> &Self::Data {
        self
    }
}
