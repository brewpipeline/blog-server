use blog_server_services::traits::author_service::Author;

use crate::utils::post_access;

use super::request_content::DeletePostRequestContent;
use super::response_content_failure::DeletePostResponseContentFailure;
use super::response_content_failure::DeletePostResponseContentFailure::*;
use super::response_content_success::DeletePostResponseContentSuccess;

pub async fn http_handler(
    (
        author,
        DeletePostRequestContent {
            id,
            post_service,
            comment_service,
        },
    ): (Author, DeletePostRequestContent),
) -> Result<DeletePostResponseContentSuccess, DeletePostResponseContentFailure> {
    let post = post_service.post_by_id(&id).await?.ok_or(NotFound)?;

    post_access::may_edit(&author, &post)?;

    comment_service.delete_by_post_id(&id).await?;

    post_service.delete_post_by_id(&id).await?;

    Ok(DeletePostResponseContentSuccess)
}
