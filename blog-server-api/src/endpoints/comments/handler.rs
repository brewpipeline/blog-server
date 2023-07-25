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
        post_service,
    },): (CommentsRequestContent,),
) -> Result<CommentsResponseContentSuccess, CommentsResponseContentFailure> {
    let post_id = post_id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    if post_id == 0 {
        return Err(IncorrectIdFormat {
            reason: String::from("should not be equal to zero"),
        });
    }

    let post = post_service
        .post_by_id(&post_id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(PostNotFound)?;

    let offset = offset.unwrap_or(0).max(0);
    let limit = limit.unwrap_or(200).max(0).min(200);

    let (comments_result, total_result) = tokio::join!(
        comment_service.comments_by_post_id(&post.id, &offset, &limit),
        comment_service.comments_count_by_post_id(&post.id),
    );

    let comments = comments_result
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .into_iter()
        .map(|a| a.into())
        .collect();

    let total = total_result.map_err(|e| DatabaseError {
        reason: e.to_string(),
    })?;

    Ok(CommentsResponseContentSuccess {
        comments,
        total,
        offset,
        limit,
    })
}
