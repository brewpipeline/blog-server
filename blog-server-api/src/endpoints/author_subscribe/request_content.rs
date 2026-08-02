use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::AuthorService;
use blog_server_services::traits::social_service::SocialService;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::AuthorSubscribeResponseContentFailure)]
pub struct AuthorSubscribeRequestContent {
    #[request(path = "id")]
    pub(super) id: u64,
    #[request(extension)]
    pub(super) social_service: Arc<dyn SocialService>,
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
