use hyper::StatusCode;

pub enum UploadImageError {
    Unauthorized(String),
    Forbidden,
    InvalidData(String),
    IoError(String),
}

impl UploadImageError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            UploadImageError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            UploadImageError::Forbidden => StatusCode::FORBIDDEN,
            UploadImageError::InvalidData(_) => StatusCode::BAD_REQUEST,
            UploadImageError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            UploadImageError::Unauthorized(reason) => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            UploadImageError::Forbidden => "insufficient rights".to_string(),
            UploadImageError::InvalidData(reason) => format!("invalid data: {}", reason),
            UploadImageError::IoError(reason) => {
                if cfg!(debug_assertions) {
                    format!("io error: {}", reason)
                } else {
                    "io error".to_string()
                }
            }
        }
    }
}
