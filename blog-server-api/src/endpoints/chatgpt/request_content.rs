use super::response_content_failure::ChatResponseContentFailure;
use blog_generic::entities::ChatQuestion;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::{
    entity_post_service::EntityPostService, post_service::PostService,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(ApiRequest)]
#[request(failure = ChatResponseContentFailure)]
pub struct ChatGptRequestContent {
    #[request(data)]
    pub(super) question: ChatQuestion,
    #[request(extension)]
    pub(super) post_service: Arc<dyn PostService>,
    #[request(extension)]
    pub(super) entity_post_service: Arc<dyn EntityPostService>,
    #[request(header = "X-Forwarded-For", or = "X-Real-IP", default = "0.0.0.0")]
    pub(super) ip: String,
    #[request(header = "User-Agent", default = "unknown")]
    pub(super) user_agent: String,
    #[request(header = "Accept-Language", default = "unknown")]
    pub(super) accept_language: String,
    #[request(header = "Chat-Session-Id")]
    pub(super) chat_session_id: Uuid,
}
