use super::request_content::AuthorBlockRequestContent;
use super::response_content_failure::AuthorBlockResponseContentFailure;
use super::response_content_failure::AuthorBlockResponseContentFailure::*;
use super::response_content_success::AuthorBlockResponseContentSuccess;

pub async fn http_handler_block(
    (request_content,): (AuthorBlockRequestContent,),
) -> Result<AuthorBlockResponseContentSuccess, AuthorBlockResponseContentFailure> {
    http_handler(request_content, 1).await
}

pub async fn http_handler_unblock(
    (request_content,): (AuthorBlockRequestContent,),
) -> Result<AuthorBlockResponseContentSuccess, AuthorBlockResponseContentFailure> {
    http_handler(request_content, 0).await
}

async fn http_handler(
    AuthorBlockRequestContent {
        id,
        author_service,
        auth_author_future,
    }: AuthorBlockRequestContent,
    is_blocked: u8,
) -> Result<AuthorBlockResponseContentSuccess, AuthorBlockResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.base.editor != 1 {
        return Err(Forbidden);
    }

    author_service
        .set_author_blocked_by_id(&id, &is_blocked)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(AuthorBlockResponseContentSuccess)
}
