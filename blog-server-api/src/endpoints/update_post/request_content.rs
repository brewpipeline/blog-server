use std::sync::Arc;

use blog_server_services::traits::{
    author_service::{Author, AuthorService},
    post_service::PostService,
};
use screw_api::request::ApiRequestContent;
use screw_components::{dyn_fn::DFuture, dyn_result::DResult};

use crate::{entities::CreatePost, extensions::Resolve, utils::auth};

pub struct UpdatePostRequestContent {
    pub(super) id: String,
    pub(super) updated_post_data: DResult<CreatePost>,
    pub(super) post_service: Arc<Box<dyn PostService>>,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for UpdatePostRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>> + Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = CreatePost;

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
