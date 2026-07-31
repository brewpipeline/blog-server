use blog_server_services::traits::author_service::Author;

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
    let id = id?;

    let post = post_service.post_by_id(&id).await?.ok_or(NotFound)?;

    if !(post.base.author_id == author.id || author.base.editor == 1) {
        return Err(if post.base.publish_type.is_published() {
            EditingForbidden
        } else {
            NotFound
        });
    }

    if post.base.publish_type.is_published() && author.base.editor == 0 {
        return Err(EditingForbidden);
    }

    comment_service.delete_by_post_id(&id).await?;

    post_service.delete_post_by_id(&id).await?;

    Ok(DeletePostResponseContentSuccess)
}
