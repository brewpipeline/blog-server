use super::request_content::AuthorMeRequestContent;
use super::response_content_failure::AuthorMeResponseContentFailure;
use super::response_content_failure::AuthorMeResponseContentFailure::*;
use super::response_content_success::AuthorMeResponseContentSuccess;

pub async fn http_handler(
    (AuthorMeRequestContent { auth_author_future },): (AuthorMeRequestContent,),
) -> Result<AuthorMeResponseContentSuccess, AuthorMeResponseContentFailure> {
    auth_author_future
        .await
        .map(|v| v.into())
        .map_err(|e| Unauthorized {
            reason: e.to_string(),
        })
}
