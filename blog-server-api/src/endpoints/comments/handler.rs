use blog_generic::entities::{CommentsContainer, TotalOffsetLimitContainer};

use super::request_content::CommentsRequestContent;
use super::response_content_failure::CommentsResponseContentFailure;
use super::response_content_failure::CommentsResponseContentFailure::*;
use super::response_content_success::CommentsResponseContentSuccess;

pub async fn http_handler(
    (CommentsRequestContent {
        post_id,
        offset,
        limit,
        comment_service,
        entity_comment_service,
    },): (CommentsRequestContent,),
) -> Result<CommentsResponseContentSuccess, CommentsResponseContentFailure> {
    let post_id = post_id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let offset = offset.unwrap_or(0).max(0);
    let limit = limit.unwrap_or(200).max(0).min(200);

    let (comments_result, total_result) = tokio::join!(
        comment_service.comments_by_post_id(&post_id, &offset, &limit),
        comment_service.comments_count_by_post_id(&post_id),
    );

    let comments = comments_result.map_err(|e| DatabaseError {
        reason: e.to_string(),
    })?;

    let total = total_result.map_err(|e| DatabaseError {
        reason: e.to_string(),
    })?;

    let comments_entities = entity_comment_service
        .comments_entities(comments)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(CommentsContainer {
        comments: comments_entities,
        base: TotalOffsetLimitContainer {
            total,
            offset,
            limit,
        },
    }
    .into())
}
