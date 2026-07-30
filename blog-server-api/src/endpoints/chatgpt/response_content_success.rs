use blog_generic::entities::ChatAnswer;
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "ai chat success")]
pub struct ChatResponseContentSuccess {
    chat_answer: ChatAnswer,
}

impl From<ChatAnswer> for ChatResponseContentSuccess {
    fn from(chat_answer: ChatAnswer) -> Self {
        ChatResponseContentSuccess { chat_answer }
    }
}
