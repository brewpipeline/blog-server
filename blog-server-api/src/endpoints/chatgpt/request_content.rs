use crate::extensions::Resolve;
use blog_generic::entities::ChatQuestion;
use blog_server_services::traits::{
    author_service::AuthorService, entity_post_service::EntityPostService,
    post_service::PostService,
};
use hyper::header::{ACCEPT_LANGUAGE, USER_AGENT};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_result::DResult;
use std::sync::Arc;
use uuid::Uuid;

pub struct ChatGptRequestContent {
    pub(super) question: DResult<ChatQuestion>,
    pub(super) post_service: Arc<dyn PostService>,
    pub(super) entity_post_service: Arc<dyn EntityPostService>,
    pub(super) session_key: String,
    pub(super) chat_session_id: DResult<Uuid>,
}

impl<Extensions> ApiRequestContent<Extensions> for ChatGptRequestContent
where
    Extensions: Resolve<Arc<dyn PostService>>
        + Resolve<Arc<dyn EntityPostService>>
        + Resolve<Arc<dyn AuthorService>>,
{
    type Data = ChatQuestion;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        let headers = &origin_content.http_parts.headers;
        let ip = headers
            .get("X-Forwarded-For")
            .or_else(|| headers.get("X-Real-IP"))
            .and_then(|v| v.to_str().ok())
            .unwrap_or("0.0.0.0");
        let ua = headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");
        let lang = headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");
        let chat_session_id: DResult<Uuid> = match headers.get("Chat-Session-Id") {
            Some(val) => match val.to_str() {
                Ok(s) if !s.trim().is_empty() => {
                    let trimmed = s.trim();
                    match Uuid::parse_str(trimmed) {
                        Ok(uuid) => Ok(uuid),
                        Err(_) => Err("Chat-Session-Id must be a valid UUID".into()),
                    }
                }
                _ => Err("Chat-Session-Id header is missing or empty".into()),
            },
            None => Err("Chat-Session-Id header is required".into()),
        };
        Self {
            question: origin_content.data_result,
            post_service: origin_content.extensions.resolve(),
            entity_post_service: origin_content.extensions.resolve(),
            session_key: format!("{}|{}|{}", ip, ua, lang),
            chat_session_id,
        }
    }
}
