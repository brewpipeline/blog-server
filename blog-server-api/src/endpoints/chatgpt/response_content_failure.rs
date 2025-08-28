use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum ChatResponseContentFailure {
    DatabaseError { reason: String },
    ParamsDecodeError { reason: String },
    OpenAiError { reason: String },
    SessionLimitReached,
}

impl ApiResponseContentBase for ChatResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            ChatResponseContentFailure::DatabaseError { .. } => &StatusCode::INTERNAL_SERVER_ERROR,
            ChatResponseContentFailure::ParamsDecodeError { .. } => &StatusCode::BAD_REQUEST,
            ChatResponseContentFailure::OpenAiError { .. } => &StatusCode::INTERNAL_SERVER_ERROR,
            ChatResponseContentFailure::SessionLimitReached => &StatusCode::TOO_MANY_REQUESTS,
        }
    }
}

impl ApiResponseContentFailure for ChatResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            ChatResponseContentFailure::DatabaseError { .. } => "CHAT_DATABASE_ERROR",
            ChatResponseContentFailure::ParamsDecodeError { .. } => "CHAT_PARAMS_ERROR",
            ChatResponseContentFailure::OpenAiError { .. } => "CHAT_OPENAI_ERROR",
            ChatResponseContentFailure::SessionLimitReached => "CHAT_SESSION_LIMIT_REACHED",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            ChatResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            ChatResponseContentFailure::ParamsDecodeError { reason } => {
                format!("params error: {}", reason)
            }
            ChatResponseContentFailure::OpenAiError { reason } => {
                if cfg!(debug_assertions) {
                    format!("openai error: {}", reason)
                } else {
                    "internal openai error".to_string()
                }
            }
            ChatResponseContentFailure::SessionLimitReached => {
                "session question limit reached".to_string()
            }
        })
    }
}
