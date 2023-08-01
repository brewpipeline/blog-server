use blog_generic::entities::PostContainer;
use blog_server_services::traits::post_service::Post as ServicePost;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct PostResponseContentSuccess {
    container: PostContainer,
}

impl Into<PostResponseContentSuccess> for ServicePost {
    fn into(self) -> PostResponseContentSuccess {
        PostResponseContentSuccess {
            container: PostContainer { post: self.into() },
        }
    }
}

impl ApiResponseContentBase for PostResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for PostResponseContentSuccess {
    type Data = PostContainer;

    fn identifier(&self) -> &'static str {
        "POST_FOUND"
    }

    fn description(&self) -> Option<String> {
        Some("post record found".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.container
    }
}
