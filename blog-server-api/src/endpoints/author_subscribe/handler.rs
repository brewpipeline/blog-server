use super::request_content::AuthorSubscribeRequestContent;
use super::response_content_failure::AuthorSubscribeResponseContentFailure;
use super::response_content_failure::AuthorSubscribeResponseContentFailure::*;
use super::response_content_success::AuthorSubscribeRequestContentSuccess;

pub async fn http_handler_subscribe(
    (request_content,): (AuthorSubscribeRequestContent,),
) -> Result<AuthorSubscribeRequestContentSuccess, AuthorSubscribeResponseContentFailure> {
    http_handler(request_content, 1).await
}

pub async fn http_handler_unsubscribe(
    (request_content,): (AuthorSubscribeRequestContent,),
) -> Result<AuthorSubscribeRequestContentSuccess, AuthorSubscribeResponseContentFailure> {
    http_handler(request_content, 0).await
}

async fn http_handler(
    AuthorSubscribeRequestContent {
        id,
        social_service,
        author_service,
        auth_author_future,
    }: AuthorSubscribeRequestContent,
    subscribe: u8,
) -> Result<AuthorSubscribeRequestContentSuccess, AuthorSubscribeResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let logged_in_author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    let is_same_user = logged_in_author.id == id;
    let is_user_admin = logged_in_author.base.editor == 1;

    let subscriber_author = match (is_same_user, is_user_admin) {
        (true, _) => logged_in_author,
        (false, true) => author_service
            .author_by_id(&id)
            .await
            .map_err(|e| DatabaseError {
                reason: e.to_string(),
            })?
            .ok_or_else(|| NotFound)?,
        (false, false) => Err(Forbidden)?,
    };

    social_service
        .set_subscribe_for_author(&subscriber_author, &subscribe)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(AuthorSubscribeRequestContentSuccess)
}
