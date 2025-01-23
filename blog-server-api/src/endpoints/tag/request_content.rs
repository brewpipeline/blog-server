use crate::extensions::Resolve;
use blog_server_services::traits::post_service::PostService;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct TagRequestContent {
    pub(super) id: String,
    pub(super) post_service: Arc<dyn PostService>,
}

impl<Extensions> ApiRequestContent<Extensions> for TagRequestContent
where
    Extensions: Resolve<Arc<dyn PostService>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            id: origin_content
                .path
                .get("id")
                .map(|n| n.to_owned())
                .unwrap_or_default(),
            post_service: origin_content.extensions.resolve(),
        }
    }
}
