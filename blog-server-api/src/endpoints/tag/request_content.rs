use crate::extensions::Resolve;
use blog_server_services::traits::post_service::PostService;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct TagRequestContent {
    pub(crate) id: String,
    pub(crate) post_service: Arc<Box<dyn PostService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for TagRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>>,
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
