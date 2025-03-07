use crate::extensions::Resolve;
use blog_server_services::traits::author_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct AuthorRequestContent {
    pub(super) slug: String,
    pub(super) author_service: Arc<dyn AuthorService>,
}

impl<Extensions> ApiRequestContent<Extensions> for AuthorRequestContent
where
    Extensions: Resolve<Arc<dyn AuthorService>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            slug: origin_content
                .path
                .get("slug")
                .map(|n| n.to_owned())
                .unwrap_or_default(),
            author_service: origin_content.extensions.resolve(),
        }
    }
}
