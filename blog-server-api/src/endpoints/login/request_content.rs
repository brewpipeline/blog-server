use blog_generic::entities::LoginQuestion;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::*;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::LoginResponseContentFailure)]
pub struct LoginRequestContent {
    #[request(data)]
    pub(super) login_question: LoginQuestion,
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
