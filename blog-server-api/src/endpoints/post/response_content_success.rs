use blog_generic::entities::{Post, PostContainer};
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct PostResponseContentSuccess {
    pub(crate) container: PostContainer,
}

impl Into<PostResponseContentSuccess> for Post {
    fn into(self) -> PostResponseContentSuccess {
        PostResponseContentSuccess {
            container: PostContainer { post: self },
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
