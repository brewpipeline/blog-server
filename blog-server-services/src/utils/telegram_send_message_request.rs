pub struct TelegramSendMessageRequest {
    pub bot_token: String,
}

impl TelegramSendMessageRequest {
    pub async fn send(&self, chat_id: &i64, text: &str) {
        let _ = reqwest::Client::new()
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
