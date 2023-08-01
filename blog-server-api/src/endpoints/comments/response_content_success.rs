use blog_generic::entities::CommentsContainer;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct CommentsResponseContentSuccess {
    pub(super) container: CommentsContainer,
}

impl Into<CommentsResponseContentSuccess> for CommentsContainer {
    fn into(self) -> CommentsResponseContentSuccess {
        CommentsResponseContentSuccess { container: self }
    }
}

impl ApiResponseContentBase for CommentsResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for CommentsResponseContentSuccess {
    type Data = CommentsContainer;

    fn identifier(&self) -> &'static str {
        "COMMENTS_OK"
    }

    fn description(&self) -> Option<String> {
        Some("comments list returned".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.container
    }
}
