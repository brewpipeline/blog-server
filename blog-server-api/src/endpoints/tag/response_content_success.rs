use blog_generic::entities::TagContainer;
use blog_server_services::traits::post_service::Tag as ServiceTag;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct TagResponseContentSuccess {
    pub(crate) container: TagContainer,
}

impl Into<TagResponseContentSuccess> for ServiceTag {
    fn into(self) -> TagResponseContentSuccess {
        TagResponseContentSuccess {
            container: TagContainer { tag: self.into() },
        }
    }
}

impl ApiResponseContentBase for TagResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for TagResponseContentSuccess {
    type Data = TagContainer;

    fn identifier(&self) -> &'static str {
        "TAG_FOUND"
    }

    fn description(&self) -> Option<String> {
        Some("tag record found".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.container
    }
}
