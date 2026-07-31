use blog_generic::{entities::CommonPost, events::NewPostPublished};
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::{
    Publish, entity_post_service::EntityPostService, post_service::PostService,
};
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::UpdatePostContentFailure)]
pub struct UpdatePostRequestContent {
    #[request(path = "id")]
    pub(super) id: u64,
    #[request(data)]
    pub(super) updated_post_data: CommonPost,
    #[request(extension)]
    pub(super) post_service: Arc<dyn PostService>,
    #[request(extension)]
    pub(super) entity_post_service: Arc<dyn EntityPostService>,
    #[request(extension)]
    pub(super) new_post_service: Arc<dyn Publish<NewPostPublished>>,
}
