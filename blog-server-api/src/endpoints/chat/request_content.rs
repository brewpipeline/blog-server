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

pub struct ChatRequestContent {
    pub(super) question: DResult<ChatQuestion>,
    pub(super) post_service: Arc<dyn PostService>,
    pub(super) entity_post_service: Arc<dyn EntityPostService>,
    pub(super) session_key: String,
}

impl<Extensions> ApiRequestContent<Extensions> for ChatRequestContent
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
        Self {
            question: origin_content.data_result,
            post_service: origin_content.extensions.resolve(),
            entity_post_service: origin_content.extensions.resolve(),
            session_key: format!("{}|{}|{}", ip, ua, lang),
        }
    }
}
