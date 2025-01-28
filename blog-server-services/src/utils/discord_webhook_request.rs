pub struct DiscordWebhookRequest {
    pub webhook_id: String,
    pub webhook_token: String,
    pub username: String,
    pub avatar_url: String,
}

impl DiscordWebhookRequest {
    pub async fn send(&self, content: &str) {
        let _ = reqwest::Client::new()
            .post(format!(
                "https://discord.com/api/webhooks/{WEBHOOK_ID}/{WEBHOOK_TOKEN}",
                WEBHOOK_ID = self.webhook_id,
                WEBHOOK_TOKEN = self.webhook_token,
            ))
            .json(&serde_json::json!({
                "avatar_url": self.avatar_url,
                "username": self.username,
                "content": content,
            }))
            .send()
            .await;
    }
}
