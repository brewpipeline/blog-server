use crate::utils::http_client;
use crate::utils::message_sink::MessageSink;

pub struct TelegramSendMessageRequest {
    pub bot_token: String,
}

impl TelegramSendMessageRequest {
    pub async fn send(&self, chat_id: &i64, text: &str) {
        let _ = http_client::shared()
            .post(format!(
                "https://api.telegram.org/bot{BOT_TOKEN}/sendMessage",
                BOT_TOKEN = self.bot_token
            ))
            .json(&serde_json::json!({
                "chat_id": chat_id,
                "text":  text,
            }))
            .send()
            .await;
    }
}

pub struct TelegramChat {
    request: TelegramSendMessageRequest,
    chat_id: i64,
}

impl TelegramChat {
    pub fn new(bot_token: String, chat_id: i64) -> Self {
        Self {
            request: TelegramSendMessageRequest { bot_token },
            chat_id,
        }
    }
}

#[async_trait]
impl MessageSink for TelegramChat {
    async fn send(&self, message: &str) {
        self.request.send(&self.chat_id, message).await;
    }
}
