use blog_server_services::traits::author_service::Author;

use super::request_content::PostUpdateRecommendedRequestContent;
use super::response_content_failure::PostUpdateRecommendedResponseContentFailure;
use super::response_content_failure::PostUpdateRecommendedResponseContentFailure::*;
use super::response_content_success::PostUpdateRecommendedResponseContentSuccess;

pub async fn http_handler_true(
    (_, request_content): (Author, PostUpdateRecommendedRequestContent),
) -> Result<PostUpdateRecommendedResponseContentSuccess, PostUpdateRecommendedResponseContentFailure>
{
    http_handler(request_content, 1).await
}

pub async fn http_handler_false(
    (_, request_content): (Author, PostUpdateRecommendedRequestContent),
) -> Result<PostUpdateRecommendedResponseContentSuccess, PostUpdateRecommendedResponseContentFailure>
{
    http_handler(request_content, 0).await
}

async fn http_handler(
    PostUpdateRecommendedRequestContent { id, post_service }: PostUpdateRecommendedRequestContent,
    recommended: u8,
) -> Result<PostUpdateRecommendedResponseContentSuccess, PostUpdateRecommendedResponseContentFailure>
{
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    post_service
        .set_post_recommended_by_id(&id, &recommended)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(PostUpdateRecommendedResponseContentSuccess)
}
