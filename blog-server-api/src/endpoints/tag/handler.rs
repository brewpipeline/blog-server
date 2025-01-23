use std::sync::Arc;

use blog_generic::entities::TagContainer;
use blog_server_services::traits::post_service::PostService;

use super::request_content::TagRequestContent;
use super::response_content_failure::TagResponseContentFailure;
use super::response_content_failure::TagResponseContentFailure::*;
use super::response_content_success::TagResponseContentSuccess;

pub async fn http_handler(
    (TagRequestContent { id, post_service },): (TagRequestContent,),
) -> Result<TagResponseContentSuccess, TagResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let tag = post_service
        .tag_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    Ok(tag.into())
}

pub async fn direct_handler(
    id: String,
    post_service: Arc<dyn PostService>,
) -> Option<TagContainer> {
    http_handler((TagRequestContent { id, post_service },))
        .await
        .ok()
        .map(|s| s.container)
}
