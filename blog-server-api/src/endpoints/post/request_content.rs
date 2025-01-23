use crate::{extensions::Resolve, utils::auth};
use blog_server_services::traits::{
    author_service::{Author, AuthorService},
    entity_post_service::EntityPostService,
    post_service::PostService,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_fn::DFuture;
use std::sync::Arc;

pub struct PostRequestContent {
    pub(super) id: String,
    pub(super) post_service: Arc<dyn PostService>,
    pub(super) entity_post_service: Arc<dyn EntityPostService>,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for PostRequestContent
where
    Extensions: Resolve<Arc<dyn PostService>>
        + Resolve<Arc<dyn EntityPostService>>
        + Resolve<Arc<dyn AuthorService>>,
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
            entity_post_service: origin_content.extensions.resolve(),
            auth_author_future: Box::pin(auth::author(
                &origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
        }
    }
}
