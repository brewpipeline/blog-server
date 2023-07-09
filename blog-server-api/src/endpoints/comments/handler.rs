use super::request_content::CommentsRequestContent;
use super::response_content_failure::CommentsResponseContentFailure;
use super::response_content_failure::CommentsResponseContentFailure::*;
use super::response_content_success::CommentsResponseContentSuccess;
use crate::extensions::Resolve;
use blog_server_services::traits::comment_service::CommentService;
use blog_server_services::traits::post_service::PostService;
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use std::sync::Arc;

async fn handler(
    post_slug: String,
    offset: Option<i64>,
    limit: Option<i64>,
    comment_service: Arc<Box<dyn CommentService>>,
    post_service: Arc<Box<dyn PostService>>,
) -> Result<CommentsResponseContentSuccess, CommentsResponseContentFailure> {
    if post_slug.is_empty() {
        return Err(PostSlugEmpty);
    }

    let post = post_service
        .post_by_slug(&post_slug)
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

pub async fn http_handler<Extensions>(
    request: ApiRequest<CommentsRequestContent, Extensions>,
) -> ApiResponse<CommentsResponseContentSuccess, CommentsResponseContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn CommentService>>> + Resolve<Arc<Box<dyn PostService>>>,
{
    ApiResponse::from(
        handler(
            request.content.post_slug,
            request.content.offset,
            request.content.limit,
            request.content.comment_service,
            request.content.post_service,
        )
        .await,
    )
}
