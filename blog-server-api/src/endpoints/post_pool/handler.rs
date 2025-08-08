use super::request_content::PostPoolRequestContent;
use super::response_content_failure::PostPoolResponseContentFailure;
use super::response_content_failure::PostPoolResponseContentFailure::*;
use super::response_content_success::PostPoolResponseContentSuccess;

pub async fn http_handler_add(
    (request_content,): (PostPoolRequestContent,),
) -> Result<PostPoolResponseContentSuccess, PostPoolResponseContentFailure> {
    http_handler(request_content, 1).await
}

pub async fn http_handler_remove(
    (request_content,): (PostPoolRequestContent,),
) -> Result<PostPoolResponseContentSuccess, PostPoolResponseContentFailure> {
    http_handler(request_content, 0).await
}

async fn http_handler(
    PostPoolRequestContent {
        id,
        post_service,
        auth_author_future,
    }: PostPoolRequestContent,
    recommended: u8,
) -> Result<PostPoolResponseContentSuccess, PostPoolResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.base.editor != 1 {
        return Err(Forbidden);
    }

    post_service
        .set_post_recommended_by_id(&id, &recommended)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(PostPoolResponseContentSuccess)
}
