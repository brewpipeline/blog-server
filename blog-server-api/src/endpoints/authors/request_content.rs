use crate::extensions::Resolve;
use blog_server_services::traits::author_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct AuthorsRequestContent {
    pub(super) query: Option<String>,
    pub(super) offset: Option<u64>,
    pub(super) limit: Option<u64>,
    pub(super) author_service: Arc<dyn AuthorService>,
}

impl<Extensions> ApiRequestContent<Extensions> for AuthorsRequestContent
where
    Extensions: Resolve<Arc<dyn AuthorService>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            query: origin_content.path.get("query").map(|n| n.to_owned()),
            offset: origin_content
                .query
                .get("offset")
                .map(|v| v.parse().ok())
                .flatten(),
            limit: origin_content
                .query
                .get("limit")
                .map(|v| v.parse().ok())
                .flatten(),
            author_service: origin_content.extensions.resolve(),
        }
    }
}
