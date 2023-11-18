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
        author_service,
        auth_author_future,
    }: AuthorSubscribeRequestContent,
    subscribe: u8,
) -> Result<AuthorSubscribeRequestContentSuccess, AuthorSubscribeResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.id != id && author.base.editor != 1 {
        return Err(Forbidden);
    }

    //TODO PULL INTO QUEUE

    //TODO change subscription state

    Ok(AuthorSubscribeRequestContentSuccess)
}
