use super::request_content::UploadImageRequestContent;
use super::response_content_failure::UploadImageResponseContentFailure;
use super::response_content_failure::UploadImageResponseContentFailure::*;
use super::response_content_success::{
    UploadImageResponseContentSuccess, UploadImageResponseContentSuccessData,
};
use crate::extensions::ExtensionsProviderType;
use futures_util::stream::once;
use hyper::body::Bytes;
use image::ImageFormat;
use multer::Multipart;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_api::response::{ApiResponse, ApiResponseContentBase};
use screw_core::request::Request;
use screw_core::response::Response;
use screw_core::routing::router;
use tokio::fs;

pub async fn handle(
    (UploadImageRequestContent {
        upload_data,
        content_type,
        auth_author_future,
    },): (UploadImageRequestContent,),
) -> Result<UploadImageResponseContentSuccess, UploadImageResponseContentFailure> {
    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.base.editor != 1 {
        return Err(Forbidden);
    }

    let bytes = upload_data.map_err(|e| InvalidData {
        reason: e.to_string(),
    })?;

    let content_type = content_type.ok_or(InvalidData {
        reason: "missing content type".to_string(),
    })?;

    let boundary = content_type
        .split("boundary=")
        .nth(1)
        .ok_or(InvalidData {
            reason: "missing boundary".to_string(),
        })?
        .to_string();

    let stream = once(async move { Ok::<Bytes, std::io::Error>(Bytes::from(bytes)) });
    let mut multipart = Multipart::new(stream, boundary);

    let field = multipart
        .next_field()
        .await
        .map_err(|e| InvalidData {
            reason: e.to_string(),
        })?
        .ok_or(InvalidData {
            reason: "file field is required".to_string(),
        })?;

    let filename = field
        .file_name()
        .map(|s| s.to_string())
        .ok_or(InvalidData {
            reason: "filename is required".to_string(),
        })?;

    let extension = filename
        .rsplit('.')
        .next()
        .ok_or(InvalidData {
            reason: "invalid filename".to_string(),
        })?
        .to_lowercase();
    if extension != "jpg" && extension != "jpeg" && extension != "png" {
        return Err(InvalidData {
            reason: "only jpg and png images are allowed".to_string(),
        });
    }

    let data = field.bytes().await.map_err(|e| InvalidData {
        reason: e.to_string(),
    })?;

    let format = image::guess_format(&data).map_err(|_| InvalidData {
        reason: "unsupported image format".to_string(),
    })?;

    match format {
        ImageFormat::Jpeg | ImageFormat::Png => {}
        _ => {
            return Err(InvalidData {
                reason: "only jpg and png images are allowed".to_string(),
            });
        }
    }

    fs::create_dir_all("images").await.map_err(|e| IoError {
        reason: e.to_string(),
    })?;

    let path = format!("images/{}", filename);
    fs::write(&path, &data).await.map_err(|e| IoError {
        reason: e.to_string(),
    })?;

    Ok(UploadImageResponseContentSuccess {
        data: UploadImageResponseContentSuccessData {
            url: format!("/images/{}", filename),
        },
    })
}

pub async fn http_handler<Extensions: ExtensionsProviderType>(
    routed_request: router::RoutedRequest<Request<Extensions>>,
) -> Response {
    let (http_parts, http_body) = routed_request.origin.http.into_parts();
    let bytes = hyper::body::to_bytes(http_body)
        .await
        .unwrap_or_else(|_| Bytes::new())
        .to_vec();

    let origin_content = ApiRequestOriginContent {
        path: routed_request.path,
        query: routed_request.query,
        http_parts,
        remote_addr: routed_request.origin.remote_addr,
        extensions: routed_request.origin.extensions.clone(),
        data_result: Ok(bytes),
    };

    let request_content = UploadImageRequestContent::create(origin_content);

    let api_response = match handle((request_content,)).await {
        Ok(success) => ApiResponse::success(success),
        Err(failure) => ApiResponse::failure(failure),
    };

    let content = api_response.content;
    let status_code = content.status_code();
    let json_bytes = serde_json::to_vec(&content).unwrap_or_default();

    let http_response = hyper::Response::builder()
        .status(status_code)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(hyper::Body::from(json_bytes))
        .unwrap();

    Response {
        http: http_response,
    }
}
