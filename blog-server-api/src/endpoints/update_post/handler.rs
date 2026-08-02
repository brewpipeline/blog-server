use blog_server_services::traits::author_service::Author;
use validator::Validate;

use crate::utils::{post_access, post_publication};

use super::request_content::UpdatePostRequestContent;
use super::response_content_failure::UpdatePostContentFailure;
use super::response_content_failure::UpdatePostContentFailure::*;
use super::response_content_success::UpdatePostContentSuccess;

pub async fn http_handler(
    (
        author,
        UpdatePostRequestContent {
            id,
            updated_post_data: base_post,
            post_service,
            entity_post_service,
            new_post_service,
        },
    ): (Author, UpdatePostRequestContent),
) -> Result<UpdatePostContentSuccess, UpdatePostContentFailure> {
    let existing_post = post_service.post_by_id(&id).await?.ok_or(NotFound)?;

    post_access::may_edit(&author, &existing_post)?;

    base_post.validate().map_err(|error| ValidationError {
        reason: error.to_string(),
    })?;

    post_access::may_publish(&author, &base_post.publish_type)
        .map_err(|reason| ValidationError { reason })?;

    let tag_titles: Vec<String> = base_post.tags.to_owned();
    let is_published_changed = base_post.publish_type != existing_post.base.publish_type;

    post_service
        .update_post_by_id(
            &id,
            &From::from((author.id, base_post)),
            &is_published_changed,
            tag_titles,
        )
        .await?;

    let updated_post = post_service.post_by_id(&id).await?.ok_or(NotFound)?;

    let updated_post_entity = entity_post_service
        .posts_entities(vec![updated_post])
        .await?
        .remove(0);

    post_publication::announce(
        new_post_service,
        &updated_post_entity,
        existing_post.base.publish_type.is_published(),
    );

    Ok(updated_post_entity.into())
}
