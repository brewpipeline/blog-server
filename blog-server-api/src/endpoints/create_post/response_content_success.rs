use blog_generic::entities::{Post, PostContainer};
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct CreatePostContentSuccess {
    container: PostContainer,
}

impl Into<CreatePostContentSuccess> for Post {
    fn into(self) -> CreatePostContentSuccess {
        CreatePostContentSuccess {
            container: PostContainer { post: self },
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
