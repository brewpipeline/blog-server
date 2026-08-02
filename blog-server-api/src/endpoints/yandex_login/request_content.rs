use blog_generic::entities::LoginYandexQuestion;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::social_service::SocialService;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::LoginYandexResponseContentFailure)]
pub struct LoginYandexRequestContent {
    #[request(data)]
    pub(super) login_yandex_question: LoginYandexQuestion,
    #[request(extension)]
    pub(super) social_service: Arc<dyn SocialService>,
}
