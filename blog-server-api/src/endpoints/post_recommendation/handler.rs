use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::PostService;

use super::request_content::PostRecommendationRequestContent;
use super::response_content_failure::PostRecommendationResponseContentFailure;
use super::response_content_failure::PostRecommendationResponseContentFailure::*;
use super::response_content_success::PostRecommendationResponseContentSuccess;

pub async fn http_handler(
    (PostRecommendationRequestContent {
        id,
        post_service,
        entity_post_service,
    },): (PostRecommendationRequestContent,),
) -> Result<PostRecommendationResponseContentSuccess, PostRecommendationResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let post = post_service
        .random_recommended_post(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    let post_entity = entity_post_service
        .posts_entities(vec![post])
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .remove(0);

    Ok(post_entity.into())
}
