use blog_generic::entities::ChatAnswer;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct ChatResponseContentSuccess {
    chat_answer: ChatAnswer,
}

impl From<ChatAnswer> for ChatResponseContentSuccess {
    fn from(chat_answer: ChatAnswer) -> Self {
        ChatResponseContentSuccess { chat_answer }
    }
}

impl ApiResponseContentBase for ChatResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for ChatResponseContentSuccess {
    type Data = ChatAnswer;

    fn identifier(&self) -> &'static str {
        "CHAT_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some("ai chat success".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.chat_answer
    }
}
