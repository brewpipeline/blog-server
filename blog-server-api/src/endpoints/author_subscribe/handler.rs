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

    let logged_in_author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    let is_same_user = logged_in_author.id == id;
    let is_user_admin = logged_in_author.base.editor == 1;

    if !is_same_user && !is_user_admin {
        return Err(Forbidden);
    }

    let subscriber_telegram_id = match (is_same_user, is_user_admin) {
        (true, _) => logged_in_author.base.telegram_id,
        (false, true) => {
            let author = author_service
                .author_by_id(&id)
                .await
                .map_err(|e| DatabaseError {
                    reason: e.to_string(),
                })?;
            match author {
                Some(a) => a.base.telegram_id,
                None => None,
            }
        }
        _ => None,
    };

    if let Some(telegram_id) = subscriber_telegram_id {
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
