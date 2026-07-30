use blog_server_services::traits::author_service::Author;

use super::request_content::AuthorMeRequestContent;
use super::response_content_failure::AuthorMeResponseContentFailure;
use super::response_content_success::AuthorMeResponseContentSuccess;

pub async fn http_handler(
    (author, AuthorMeRequestContent): (Author, AuthorMeRequestContent),
) -> Result<AuthorMeResponseContentSuccess, AuthorMeResponseContentFailure> {
    Ok(author.into())
}
