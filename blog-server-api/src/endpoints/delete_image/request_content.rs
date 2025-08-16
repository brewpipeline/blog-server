use crate::{extensions::Resolve, utils::auth};
use blog_server_services::traits::author_service::{Author, AuthorService};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_fn::DFuture;
use std::sync::Arc;

pub struct DeleteImageRequestContent {
    pub(super) filename: String,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for DeleteImageRequestContent
where
    Extensions: Resolve<Arc<dyn AuthorService>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            filename: origin_content
                .path
                .get("filename")
                .map(|n| n.to_owned())
                .unwrap_or_default(),
            auth_author_future: Box::pin(auth::author(
                &origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
        }
    }
}

