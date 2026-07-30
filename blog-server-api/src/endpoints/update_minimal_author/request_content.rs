use blog_generic::entities::CommonMinimalAuthor;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::AuthorService;
use screw_components::dyn_result::DResult;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct UpdateMinimalAuthorRequestContent {
    #[request(data)]
    pub(super) updated_minimal_author_data: DResult<CommonMinimalAuthor>,
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
