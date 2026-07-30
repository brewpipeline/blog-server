use blog_generic::entities::TagContainer;
use blog_server_api_macros::ApiSuccess;
use blog_server_services::traits::post_service::Tag as ServiceTag;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, description = "tag record found")]
pub struct TagResponseContentSuccess {
    pub(super) container: TagContainer,
}

impl From<ServiceTag> for TagResponseContentSuccess {
    fn from(value: ServiceTag) -> Self {
        TagResponseContentSuccess {
            container: TagContainer { tag: value.into() },
        }
    }
}
