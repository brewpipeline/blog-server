use crate::{extensions::Resolve, utils::auth};
use blog_generic::{entities::CommonPost, events::NewPostPublished};
use blog_server_services::traits::{
    author_service::{Author, AuthorService},
    entity_post_service::EntityPostService,
    post_service::PostService,
    Publish,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::{dyn_fn::DFuture, dyn_result::DResult};
use std::sync::Arc;

pub struct CreatePostRequestContent {
    pub(super) new_post_data: DResult<CommonPost>,
    pub(super) post_service: Arc<dyn PostService>,
    pub(super) entity_post_service: Arc<dyn EntityPostService>,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
    pub(super) new_post_service: Arc<dyn Publish<NewPostPublished>>,
}

impl<Extensions> ApiRequestContent<Extensions> for CreatePostRequestContent
where
    Extensions: Resolve<Arc<dyn PostService>>
        + Resolve<Arc<dyn AuthorService>>
        + Resolve<Arc<dyn EntityPostService>>
        + Resolve<Arc<dyn Publish<NewPostPublished>>>,
{
    type Data = CommonPost;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            new_post_data: origin_content.data_result,
            post_service: origin_content.extensions.resolve(),
            entity_post_service: origin_content.extensions.resolve(),
            auth_author_future: Box::pin(auth::author(
                &origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
            new_post_service: origin_content.extensions.resolve(),
        }
    }
}
