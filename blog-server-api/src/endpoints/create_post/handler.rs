use blog_server_services::traits::author_service::Author;
use validator::Validate;

use crate::utils::{post_access, post_publication};

use super::request_content::CreatePostRequestContent;
use super::response_content_failure::CreatePostContentFailure;
use super::response_content_failure::CreatePostContentFailure::*;
use super::response_content_success::CreatePostContentSuccess;

pub async fn http_handler(
    (
        author,
        CreatePostRequestContent {
            new_post_data: base_post,
            post_service,
            entity_post_service,
            new_post_service,
        },
    ): (Author, CreatePostRequestContent),
) -> Result<CreatePostContentSuccess, CreatePostContentFailure> {
    base_post.validate().map_err(|error| ValidationError {
        reason: error.to_string(),
    })?;

    post_access::may_publish(&author, &base_post.publish_type)
        .map_err(|reason| ValidationError { reason })?;

    let tag_titles: Vec<String> = base_post.tags.to_owned();

    let inserted_id = post_service
        .create_post(&From::from((author.id, base_post)), tag_titles)
        .await?;

    let created_post = post_service
        .post_by_id(&inserted_id)
        .await?
        .ok_or(InsertFailed)?;

    let created_post_entity = entity_post_service
        .posts_entities(vec![created_post])
        .await?
        .remove(0);

    post_publication::announce(new_post_service, &created_post_entity, false);

    Ok(created_post_entity.into())
}
