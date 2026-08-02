use blog_generic::entities::CommonSecondaryAuthor;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::AuthorService;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::UpdateSecondaryAuthorContentFailure)]
pub struct UpdateSecondaryAuthorRequestContent {
    #[request(data)]
    pub(super) updated_secondary_author_data: CommonSecondaryAuthor,
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
