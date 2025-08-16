use super::request_content::PostUpdateRecommendedRequestContent;
use super::response_content_failure::PostUpdateRecommendedResponseContentFailure;
use super::response_content_failure::PostUpdateRecommendedResponseContentFailure::*;
use super::response_content_success::PostUpdateRecommendedResponseContentSuccess;

pub async fn http_handler_true(
    (request_content,): (PostUpdateRecommendedRequestContent,),
) -> Result<PostUpdateRecommendedResponseContentSuccess, PostUpdateRecommendedResponseContentFailure>
{
    http_handler(request_content, 1).await
}

pub async fn http_handler_false(
    (request_content,): (PostUpdateRecommendedRequestContent,),
) -> Result<PostUpdateRecommendedResponseContentSuccess, PostUpdateRecommendedResponseContentFailure>
{
    http_handler(request_content, 0).await
}

async fn http_handler(
    PostUpdateRecommendedRequestContent {
        id,
        post_service,
        auth_author_future,
    }: PostUpdateRecommendedRequestContent,
    recommended: u8,
) -> Result<PostUpdateRecommendedResponseContentSuccess, PostUpdateRecommendedResponseContentFailure>
{
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

    Ok(PostUpdateRecommendedResponseContentSuccess)
}
