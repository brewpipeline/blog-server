use std::sync::Arc;

use blog_generic::entities::PostContainer;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::PostService;

use crate::utils::auth;

use super::request_content::PostRequestContent;
use super::response_content_failure::PostResponseContentFailure;
use super::response_content_failure::PostResponseContentFailure::*;
use super::response_content_success::PostResponseContentSuccess;

pub async fn http_handler(
    (PostRequestContent {
        id,
        post_service,
        entity_post_service,
        auth_author_future,
    },): (PostRequestContent,),
) -> Result<PostResponseContentSuccess, PostResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let post = post_service
        .post_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    if !post.base.publish_type.is_published() {
        let have_access = if let Some(author) = auth_author_future.await.ok() {
            post.base.author_id == author.id || author.base.editor == 1
        } else {
            false
        };
        if !have_access {
            return Err(NotFound);
        }
    }

    let post_entity = entity_post_service
        .posts_entities(vec![post])
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .remove(0);

    Ok(post_entity.into())
}

pub async fn direct_handler(
    id: String,
    post_service: Arc<Box<dyn PostService>>,
    entity_post_service: Arc<Box<dyn EntityPostService>>,
) -> Option<PostContainer> {
    http_handler((PostRequestContent {
        id,
        post_service,
        entity_post_service,
        auth_author_future: Box::pin(std::future::ready(Err(auth::Error::TokenMissing))),
    },))
    .await
    .ok()
    .map(|s| s.container)
}
