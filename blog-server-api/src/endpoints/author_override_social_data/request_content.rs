use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::*;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct AuthorOverrideSocialDataRequestContent {
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
