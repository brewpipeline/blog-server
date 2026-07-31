use blog_server_services::traits::author_service::Author;

use super::request_content::AuthorBlockRequestContent;
use super::response_content_failure::AuthorBlockResponseContentFailure;
use super::response_content_failure::AuthorBlockResponseContentFailure::*;
use super::response_content_success::AuthorBlockResponseContentSuccess;

pub async fn http_handler_block(
    (_, request_content): (Author, AuthorBlockRequestContent),
) -> Result<AuthorBlockResponseContentSuccess, AuthorBlockResponseContentFailure> {
    http_handler(request_content, 1).await
}

pub async fn http_handler_unblock(
    (_, request_content): (Author, AuthorBlockRequestContent),
) -> Result<AuthorBlockResponseContentSuccess, AuthorBlockResponseContentFailure> {
    http_handler(request_content, 0).await
}

async fn http_handler(
    AuthorBlockRequestContent { id, author_service }: AuthorBlockRequestContent,
    is_blocked: u8,
) -> Result<AuthorBlockResponseContentSuccess, AuthorBlockResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    author_service
        .set_author_blocked_by_id(&id, &is_blocked)
        .await?;

    Ok(AuthorBlockResponseContentSuccess)
}
