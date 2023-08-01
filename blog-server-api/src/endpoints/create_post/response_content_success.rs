use blog_generic::entities::PostContainer;
use blog_server_services::traits::post_service::Post as ServicePost;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct CreatePostContentSuccess {
    container: PostContainer,
}

impl Into<CreatePostContentSuccess> for ServicePost {
    fn into(self) -> CreatePostContentSuccess {
        CreatePostContentSuccess {
            container: PostContainer { post: self.into() },
        }
    }
}

impl ApiResponseContentBase for CreatePostContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for CreatePostContentSuccess {
    type Data = PostContainer;

    fn identifier(&self) -> &'static str {
        "POST_CREATED"
    }

    fn description(&self) -> Option<String> {
        Some(String::from("post record created"))
    }

    fn data(&self) -> &Self::Data {
        &self.container
    }
}
