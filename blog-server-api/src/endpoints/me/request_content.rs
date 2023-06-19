use crate::extensions::Resolve;
use crate::utils::login;
use blog_server_services::traits::author_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_fn::DFuture;
use std::sync::Arc;

pub struct MeRequestContent {
    pub(super) self_author_fut: DFuture<Result<Author, login::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for MeRequestContent
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            self_author_fut: Box::pin(login::author(
                origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
        }
    }
}
