use blog_generic::entities::LoginQuestion;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::*;
use screw_components::dyn_result::DResult;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct LoginRequestContent {
    #[request(data)]
    pub(super) login_question: DResult<LoginQuestion>,
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
