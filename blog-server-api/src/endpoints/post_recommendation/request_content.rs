use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::{
    entity_post_service::EntityPostService, post_service::PostService,
};
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::PostRecommendationResponseContentFailure)]
pub struct PostRecommendationRequestContent {
    #[request(path = "id")]
    pub(super) id: u64,
    #[request(extension)]
    pub(super) post_service: Arc<dyn PostService>,
    #[request(extension)]
    pub(super) entity_post_service: Arc<dyn EntityPostService>,
}
