use blog_generic::entities::LoginYandexQuestion;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::social_service::SocialService;
use screw_components::dyn_result::DResult;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct LoginYandexRequestContent {
    #[request(data)]
    pub(super) login_yandex_question: DResult<LoginYandexQuestion>,
    #[request(extension)]
    pub(super) social_service: Arc<dyn SocialService>,
}
