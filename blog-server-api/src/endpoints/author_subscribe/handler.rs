use blog_server_services::traits::author_service::Author;

use super::request_content::AuthorSubscribeRequestContent;
use super::response_content_failure::AuthorSubscribeResponseContentFailure;
use super::response_content_failure::AuthorSubscribeResponseContentFailure::*;
use super::response_content_success::AuthorSubscribeRequestContentSuccess;

pub async fn http_handler_subscribe(
    (author, request_content): (Author, AuthorSubscribeRequestContent),
) -> Result<AuthorSubscribeRequestContentSuccess, AuthorSubscribeResponseContentFailure> {
    http_handler(author, request_content, 1).await
}

pub async fn http_handler_unsubscribe(
    (author, request_content): (Author, AuthorSubscribeRequestContent),
) -> Result<AuthorSubscribeRequestContentSuccess, AuthorSubscribeResponseContentFailure> {
    http_handler(author, request_content, 0).await
}

async fn http_handler(
    logged_in_author: Author,
    AuthorSubscribeRequestContent {
        id,
        social_service,
        author_service,
    }: AuthorSubscribeRequestContent,
    subscribe: u8,
) -> Result<AuthorSubscribeRequestContentSuccess, AuthorSubscribeResponseContentFailure> {
    let id = id?;

    let is_same_user = logged_in_author.id == id;
    let is_user_admin = logged_in_author.base.editor == 1;

    let subscriber_author = match (is_same_user, is_user_admin) {
        (true, _) => logged_in_author,
        (false, true) => author_service
            .author_by_id(&id)
            .await?
            .ok_or_else(|| NotFound)?,
        (false, false) => Err(Forbidden)?,
    };

    social_service
        .set_subscribe_for_author(&subscriber_author, &subscribe)
        .await?;

    Ok(AuthorSubscribeRequestContentSuccess)
}
