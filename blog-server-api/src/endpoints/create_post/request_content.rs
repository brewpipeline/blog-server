use crate::{entities::PostRequestData, extensions::Resolve, utils::auth};
use blog_server_services::traits::{
    author_service::{Author, AuthorService},
    post_service::PostService,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::{dyn_fn::DFuture, dyn_result::DResult};
use std::sync::Arc;

pub struct CreatePostRequestContent {
    pub(super) new_post_data: DResult<PostRequestData>,
    pub(super) post_service: Arc<Box<dyn PostService>>,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for CreatePostRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>> + Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = PostRequestData;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            new_post_data: origin_content.data_result,
            post_service: origin_content.extensions.resolve(),
            auth_author_future: Box::pin(auth::author(
                origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
        }
    }
}
