use super::types::{UploadImageError, UploadImageRequest};
use futures_util::stream::once;
use hyper::body::Bytes;
use image::ImageFormat;
use multer::Multipart;
use tokio::fs;

pub async fn http_handler(
    UploadImageRequest {
        bytes,
        content_type,
        author,
    }: UploadImageRequest,
) -> Result<(), UploadImageError> {
    let author = author.map_err(|e| UploadImageError::Unauthorized(e.to_string()))?;
    if author.base.editor != 1 {
        return Err(UploadImageError::Forbidden);
    }

    let content_type = content_type.ok_or_else(|| {
        UploadImageError::InvalidData("missing content type".to_string())
    })?;
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

    let format = image::guess_format(&data).map_err(|_| {
        UploadImageError::InvalidData("unsupported image format".to_string())
    })?;
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

    Ok(())
}
