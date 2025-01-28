use crate::traits::Publish;
use crate::utils::telegram_send_message_request::TelegramSendMessageRequest;
use blog_generic::events::NewPostPublished;
use std::sync::Arc;

pub fn create_telegram_new_post_published_service(
    bot_token: String,
    site_url: String,
    chat_id: i64,
) -> Result<Arc<impl Publish<NewPostPublished>>, Box<dyn std::error::Error + Send + Sync>> {
    if bot_token.is_empty() {
        return Err("bot token is empty".into());
    }
    Ok(Arc::new(TelegramNewPostPublishedService {
        telegram_send_message_request: TelegramSendMessageRequest { bot_token },
        site_url,
        chat_id,
    }))
}

struct TelegramNewPostPublishedService {
    telegram_send_message_request: TelegramSendMessageRequest,
    site_url: String,
    chat_id: i64,
}

#[async_trait]
impl Publish<NewPostPublished> for TelegramNewPostPublishedService {
    async fn publish(&self, event: NewPostPublished) {
        self.telegram_send_message_request
            .send(
                &self.chat_id,
                &format!(
                    "{SITE_URL}{PATH}",
                    SITE_URL = self.site_url,
                    PATH = event.post_sub_url
                ),
            )
            .await;
    }
}
