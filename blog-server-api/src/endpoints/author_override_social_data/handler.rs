use blog_server_services::traits::author_service::Author;

use super::request_content::AuthorOverrideSocialDataRequestContent;
use super::response_content_failure::AuthorOverrideSocialDataResponseContentFailure;
use super::response_content_success::AuthorOverrideSocialDataResponseContentSuccess;

pub async fn http_handler_disabled(
    (author, request_content): (Author, AuthorOverrideSocialDataRequestContent),
) -> Result<
    AuthorOverrideSocialDataResponseContentSuccess,
    AuthorOverrideSocialDataResponseContentFailure,
> {
    http_handler(author, request_content, 0).await
}

async fn http_handler(
    author: Author,
    AuthorOverrideSocialDataRequestContent { author_service }: AuthorOverrideSocialDataRequestContent,
    override_social_data: u8,
) -> Result<
    AuthorOverrideSocialDataResponseContentSuccess,
    AuthorOverrideSocialDataResponseContentFailure,
> {
    author_service
        .set_author_override_social_data_by_id(&author.id, &override_social_data)
        .await?;

    Ok(().into())
}
