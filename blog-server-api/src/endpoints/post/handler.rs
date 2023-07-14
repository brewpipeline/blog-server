use super::request_content::PostRequestContent;
use super::response_content_failure::PostResponseContentFailure;
use super::response_content_failure::PostResponseContentFailure::*;
use super::response_content_success::PostResponseContentSuccess;

pub async fn http_handler(
    (PostRequestContent { slug, post_service },): (PostRequestContent,),
) -> Result<PostResponseContentSuccess, PostResponseContentFailure> {
    if slug.is_empty() {
        return Err(SlugEmpty);
    }

    let post = post_service
        .post_by_slug(&slug)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    Ok(post.into())
}
