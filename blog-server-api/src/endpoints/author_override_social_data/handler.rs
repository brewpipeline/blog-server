use super::request_content::AuthorOverrideSocialDataRequestContent;
use super::response_content_failure::AuthorOverrideSocialDataResponseContentFailure;
use super::response_content_failure::AuthorOverrideSocialDataResponseContentFailure::*;
use super::response_content_success::AuthorOverrideSocialDataResponseContentSuccess;

pub async fn http_handler_enabled(
    (request_content,): (AuthorOverrideSocialDataRequestContent,),
) -> Result<
    AuthorOverrideSocialDataResponseContentSuccess,
    AuthorOverrideSocialDataResponseContentFailure,
> {
    http_handler(request_content, 1).await
}

pub async fn http_handler_disabled(
    (request_content,): (AuthorOverrideSocialDataRequestContent,),
) -> Result<
    AuthorOverrideSocialDataResponseContentSuccess,
    AuthorOverrideSocialDataResponseContentFailure,
> {
    http_handler(request_content, 0).await
}

async fn http_handler(
    AuthorOverrideSocialDataRequestContent {
        id,
        author_service,
        auth_author_future,
    }: AuthorOverrideSocialDataRequestContent,
    override_social_data: u8,
) -> Result<
    AuthorOverrideSocialDataResponseContentSuccess,
    AuthorOverrideSocialDataResponseContentFailure,
> {
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
        .set_author_override_social_data_by_id(&id, &override_social_data)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(AuthorOverrideSocialDataResponseContentSuccess)
}
