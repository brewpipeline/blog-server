use super::request_content::DeleteImageRequestContent;
use super::response_content_failure::DeleteImageResponseContentFailure;
use super::response_content_failure::DeleteImageResponseContentFailure::*;
use super::response_content_success::DeleteImageResponseContentSuccess;
use tokio::fs;

pub async fn http_handler(
    (DeleteImageRequestContent {
        filename,
        auth_author_future,
    },): (DeleteImageRequestContent,),
) -> Result<DeleteImageResponseContentSuccess, DeleteImageResponseContentFailure> {
    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.base.editor != 1 {
        return Err(Forbidden);
    }

    let path = format!("images/{}", filename);
    fs::remove_file(&path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            NotFound
        } else {
            IoError {
                reason: e.to_string(),
            }
        }
    })?;

    Ok(DeleteImageResponseContentSuccess)
}

