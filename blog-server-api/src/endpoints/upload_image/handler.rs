use super::types::UploadImageError;
use crate::{extensions::Resolve, utils::auth};
use blog_server_services::traits::author_service::{Author, AuthorService};
use futures_util::stream::once;
use hyper::{body::Bytes, header, Body, StatusCode};
use image::ImageFormat;
use multer::Multipart;
use screw_core::request::Request;
use screw_core::response::Response;
use screw_core::routing::router;
use std::sync::Arc;
use tokio::fs;

pub async fn http_handler<Extensions>(
    routed_request: router::RoutedRequest<Request<Extensions>>,
) -> Response
where
    Extensions: Resolve<Arc<dyn AuthorService>> + Send + Sync + 'static,
{
    let author_service: Arc<dyn AuthorService> = routed_request.origin.extensions.resolve();
    let (http_parts, http_body) = routed_request.origin.http.into_parts();
    let bytes = hyper::body::to_bytes(http_body)
        .await
        .unwrap_or_else(|_| Bytes::new())
        .to_vec();
    let content_type = http_parts
        .headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let author = auth::author(&http_parts, author_service).await;

    let result = async {
        let author: Author = author.map_err(|e| UploadImageError::Unauthorized(e.to_string()))?;
        if author.base.editor != 1 {
            return Err(UploadImageError::Forbidden);
        }

        let content_type = content_type
            .ok_or_else(|| UploadImageError::InvalidData("missing content type".to_string()))?;
        let boundary = content_type
            .split("boundary=")
            .nth(1)
            .ok_or_else(|| UploadImageError::InvalidData("missing boundary".to_string()))?
            .to_string();

        let stream = once(async move { Ok::<Bytes, std::io::Error>(Bytes::from(bytes)) });
        let mut multipart = Multipart::new(stream, boundary);

        let field = multipart
            .next_field()
            .await
            .map_err(|e| UploadImageError::InvalidData(e.to_string()))?
            .ok_or_else(|| UploadImageError::InvalidData("file field is required".to_string()))?;

        let filename = field
            .file_name()
            .map(|s| s.to_string())
            .ok_or_else(|| UploadImageError::InvalidData("filename is required".to_string()))?;

        let extension = filename
            .rsplit('.')
            .next()
            .ok_or_else(|| UploadImageError::InvalidData("invalid filename".to_string()))?
            .to_lowercase();
        if extension != "jpg" && extension != "jpeg" && extension != "png" {
            return Err(UploadImageError::InvalidData(
                "only jpg and png images are allowed".to_string(),
            ));
        }

        let data = field
            .bytes()
            .await
            .map_err(|e| UploadImageError::InvalidData(e.to_string()))?;

        let format = image::guess_format(&data)
            .map_err(|_| UploadImageError::InvalidData("unsupported image format".to_string()))?;
        match format {
            ImageFormat::Jpeg | ImageFormat::Png => {}
            _ => {
                return Err(UploadImageError::InvalidData(
                    "only jpg and png images are allowed".to_string(),
                ));
            }
        }

        fs::create_dir_all("images")
            .await
            .map_err(|e| UploadImageError::IoError(e.to_string()))?;

        let path = format!("images/{}", filename);
        fs::write(&path, &data)
            .await
            .map_err(|e| UploadImageError::IoError(e.to_string()))?;

        Ok::<(), UploadImageError>(())
    }
    .await;

    match result {
        Ok(_) => Response {
            http: hyper::Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap(),
        },
        Err(e) => Response {
            http: hyper::Response::builder()
                .status(e.status_code())
                .body(Body::from(e.message()))
                .unwrap(),
        },
    }
}
