use blog_generic::entities::PublishType;
use blog_generic::events::NewPostPublished;
use blog_server_services::traits::author_service::Author;
use validator::Validate;

use super::request_content::UpdatePostRequestContent;
use super::response_content_failure::UpdatePostContentFailure;
use super::response_content_failure::UpdatePostContentFailure::*;
use super::response_content_success::UpdatePostContentSuccess;

pub async fn http_handler(
    (
        author,
        UpdatePostRequestContent {
            id,
            updated_post_data,
            post_service,
            entity_post_service,
            new_post_service,
        },
    ): (Author, UpdatePostRequestContent),
) -> Result<UpdatePostContentSuccess, UpdatePostContentFailure> {
    let id = id?;

    let existing_post = post_service.post_by_id(&id).await?.ok_or(NotFound)?;

    if !(existing_post.base.author_id == author.id || author.base.editor == 1) {
        return Err(if existing_post.base.publish_type.is_published() {
            EditingForbidden
        } else {
            NotFound.into()
        });
    }

    if existing_post.base.publish_type.is_published() && author.base.editor == 0 {
        return Err(EditingForbidden);
    }

    let base_post = updated_post_data.map_err(|e| ValidationError {
        reason: e.to_string(),
    })?;

    if let Some(err) = base_post.validate().err() {
        return Err(ValidationError {
            reason: err.to_string(),
        }
        .into());
    }

    if author.base.editor == 0 && base_post.publish_type.is_published() {
        return Err(ValidationError {
            reason: "publishing not allowed for you".to_owned(),
        }
        .into());
    }

    let tag_titles: Vec<String> = base_post.tags.to_owned();
    let is_published_changed = base_post.publish_type != existing_post.base.publish_type;

    post_service
        .update_post_by_id(
            &id,
            &From::from((author.id, base_post)),
            &is_published_changed,
        )
        .await?;

    let post_tags = post_service.create_tags(tag_titles).await?;

    post_service.merge_post_tags(&id, post_tags).await?;

    let updated_post = post_service.post_by_id(&id).await?.ok_or(NotFound)?;

    let is_visible_published = updated_post.base.publish_type == PublishType::Published;

    let updated_post_entity = entity_post_service
        .posts_entities(vec![updated_post])
        .await?
        .remove(0);

    if !existing_post.base.publish_type.is_published() && is_visible_published {
        let new_post_published = NewPostPublished {
            blog_user_id: updated_post_entity.author.id,
            post_sub_url: format!(
                "/post/{}/{}",
                updated_post_entity.slug, updated_post_entity.id
            ),
        };
        tokio::spawn(async move { new_post_service.publish(new_post_published).await });
    }

    Ok(updated_post_entity.into())
}
