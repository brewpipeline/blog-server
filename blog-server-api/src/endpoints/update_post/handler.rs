use super::request_content::UpdatePostRequestContent;
use super::response_content_failure::UpdatePostContentFailure;
use super::response_content_failure::UpdatePostContentFailure::*;
use super::response_content_success::UpdatePostContentSuccess;

pub async fn http_handler(
    (UpdatePostRequestContent {
        id,
        updated_post_data,
        post_service,
        auth_author_future,
    },): (UpdatePostRequestContent,),
) -> Result<UpdatePostContentSuccess, UpdatePostContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    if id == 0 {
        return Err(IncorrectIdFormat {
            reason: String::from("should not be equal to zero"),
        });
    }

    let base_post = updated_post_data.map_err(|e| ValidationError {
        reason: e.to_string(),
    })?;

    let tag_titles: Vec<String> = base_post.tags.to_owned();

    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    let existing_post = post_service
        .post_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(PostNotFound)?;

    if author.base.slug != existing_post.author_slug {
        return Err(EditingForbidden);
    }

    post_service
        .update_post(&id, &From::from((author.id, base_post)))
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

    Ok(updated_post.into())
}
