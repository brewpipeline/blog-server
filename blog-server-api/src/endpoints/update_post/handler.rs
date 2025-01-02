use blog_generic::entities::PublishedType;
use blog_generic::events::NewPostPublished;
use validator::Validate;

use super::request_content::UpdatePostRequestContent;
use super::response_content_failure::UpdatePostContentFailure;
use super::response_content_failure::UpdatePostContentFailure::*;
use super::response_content_success::UpdatePostContentSuccess;

pub async fn http_handler(
    (UpdatePostRequestContent {
        id,
        updated_post_data,
        post_service,
        entity_post_service,
        auth_author_future,
        event_bus_service,
    },): (UpdatePostRequestContent,),
) -> Result<UpdatePostContentSuccess, UpdatePostContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.base.blocked == 1 {
        return Err(EditingForbidden);
    }

    let existing_post = post_service
        .post_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(PostNotFound)?;

    if !(existing_post.base.author_id == author.id || author.base.editor == 1) {
        return Err(if existing_post.base.published_type.is_published() {
            EditingForbidden
        } else {
            PostNotFound
        });
    }

    if existing_post.base.published_type.is_published() && author.base.editor == 0 {
        return Err(EditingForbidden);
    }

    let base_post = updated_post_data.map_err(|e| ValidationError {
        reason: e.to_string(),
    })?;

    if let Some(err) = base_post.validate().err() {
        return Err(ValidationError {
            reason: err.to_string(),
        });
    }

    if author.base.editor == 0 && base_post.published_type.is_published() {
        return Err(ValidationError {
            reason: "publishing not allowed for you".to_owned(),
        });
    }

    let tag_titles: Vec<String> = base_post.tags.to_owned();
    let is_published_changed = base_post.published_type != existing_post.base.published_type;

    post_service
        .update_post_by_id(
            &id,
            &From::from((author.id, base_post)),
            &is_published_changed,
        )
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    let post_tags = post_service
        .create_tags(tag_titles)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    post_service
        .merge_post_tags(&id, post_tags)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    let updated_post = post_service
        .post_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(PostNotFound)?;

    let is_visible_published = updated_post.base.published_type == PublishedType::Published;

    let updated_post_entity = entity_post_service
        .posts_entities(vec![updated_post])
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .remove(0);

    if !existing_post.base.published_type.is_published() && is_visible_published {
        let new_post_published = NewPostPublished {
            blog_user_id: updated_post_entity.author.id,
            post_sub_url: format!(
                "/post/{}/{}",
                updated_post_entity.slug, updated_post_entity.id
            ),
        };
        tokio::spawn(async move { event_bus_service.publish(new_post_published).await });
    }

    Ok(updated_post_entity.into())
}
