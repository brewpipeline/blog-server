use crate::extensions::Resolve;
use crate::utils::auth;
use blog_server_services::traits::author_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_fn::DFuture;
use std::sync::Arc;

pub struct AuthorOverrideSocialDataRequestContent {
    pub(super) author_service: Arc<Box<dyn AuthorService>>,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for AuthorOverrideSocialDataRequestContent
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            author_service: origin_content.extensions.resolve(),
            auth_author_future: Box::pin(auth::author(
                &origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
        }
    }
}
