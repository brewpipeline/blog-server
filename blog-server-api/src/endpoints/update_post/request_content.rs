use blog_generic::entities::CommonPost;
use blog_server_services::traits::{
    author_service::{Author, AuthorService},
    post_service::PostService,
};
use screw_api::request::ApiRequestContent;
use screw_components::{dyn_fn::DFuture, dyn_result::DResult};
use std::sync::Arc;

use crate::{extensions::Resolve, utils::auth};

pub struct UpdatePostRequestContent {
    pub(super) id: String,
    pub(super) updated_post_data: DResult<CommonPost>,
    pub(super) post_service: Arc<Box<dyn PostService>>,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for UpdatePostRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>> + Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = CommonPost;

    fn create(
        origin_content: screw_api::request::ApiRequestOriginContent<Self::Data, Extensions>,
    ) -> Self {
        Self {
            id: origin_content
                .path
                .get("id")
                .map(|n| n.to_owned())
                .unwrap_or_default(),
            updated_post_data: origin_content.data_result,
            post_service: origin_content.extensions.resolve(),
            auth_author_future: Box::pin(auth::author(
                origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
        }
    }
}
