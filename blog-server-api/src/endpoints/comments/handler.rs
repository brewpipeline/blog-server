use blog_generic::entities::CommentsContainer;

use crate::utils::pagination::Pagination;

use super::request_content::CommentsRequestContent;
use super::response_content_failure::CommentsResponseContentFailure;
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
    let pagination = Pagination::new(offset, limit, 200);

    let (comments_result, total_result) = tokio::join!(
        comment_service.comments_by_post_id(&post_id, &pagination.offset, &pagination.limit),
        comment_service.comments_count_by_post_id(&post_id),
    );

    let comments = comments_result?;

    let total = total_result?;

    let comments_entities = entity_comment_service.comments_entities(comments).await?;

    Ok(CommentsContainer {
        comments: comments_entities,
        base: pagination.with_total(total),
    }
    .into())
}
