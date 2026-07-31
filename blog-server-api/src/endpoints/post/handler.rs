use std::sync::Arc;

use blog_generic::entities::PostContainer;
use blog_server_services::traits::author_service::Author;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::PostService;

use super::request_content::PostRequestContent;
use super::response_content_failure::PostResponseContentFailure;
use super::response_content_failure::PostResponseContentFailure::*;
use super::response_content_success::PostResponseContentSuccess;

pub async fn http_handler(
    (
        author,
        PostRequestContent {
            id,
            post_service,
            entity_post_service,
        },
    ): (Option<Author>, PostRequestContent),
) -> Result<PostResponseContentSuccess, PostResponseContentFailure> {
    let id = id?;

    let post = post_service.post_by_id(&id).await?.ok_or(NotFound)?;

    if !post.base.publish_type.is_published() {
        let have_access = if let Some(author) = author {
            post.base.author_id == author.id || author.base.editor == 1
        } else {
            false
        };
        if !have_access {
            return Err(NotFound.into());
        }
    }

    let post_entity = entity_post_service
        .posts_entities(vec![post])
        .await?
        .remove(0);

    Ok(post_entity.into())
}

#[cfg_attr(not(feature = "ssr"), allow(dead_code))]
pub async fn direct_handler(
    id: u64,
    post_service: Arc<dyn PostService>,
    entity_post_service: Arc<dyn EntityPostService>,
) -> Option<PostContainer> {
    http_handler((
        None,
        PostRequestContent {
            id: Ok(id),
            post_service,
            entity_post_service,
        },
    ))
    .await
    .ok()
    .map(|s| s.container)
}
