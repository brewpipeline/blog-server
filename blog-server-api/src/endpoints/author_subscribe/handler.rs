use blog_generic::events::SubscriptionStateChanged;

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
        event_bus_service,
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

    if let Some(telegram_id) = author.base.telegram_id {
        let event = SubscriptionStateChanged {
            blog_user_id: id,
            user_telegram_id: telegram_id,
            new_state: subscribe,
        };

        tokio::spawn(async move { event_bus_service.publish(event).await });
    } else {
        return Err(NotSupported);
    }

    author_service
        .set_author_subscription_by_id(&id, &subscribe)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(AuthorSubscribeRequestContentSuccess)
}
