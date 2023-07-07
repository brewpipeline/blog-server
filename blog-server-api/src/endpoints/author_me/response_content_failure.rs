use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum AuthorMeResponseContentFailure {
    Unauthorized { reason: String },
}

impl ApiResponseContentBase for AuthorMeResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            AuthorMeResponseContentFailure::Unauthorized { reason: _ } => &StatusCode::UNAUTHORIZED,
        }
    }
}

impl ApiResponseContentFailure for AuthorMeResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            AuthorMeResponseContentFailure::Unauthorized { reason: _ } => "AUTHOR_ME_UNAUTHORIZED",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            AuthorMeResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
        })
    }
}
