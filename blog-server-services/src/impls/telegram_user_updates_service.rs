use blog_generic::events::{NewPostPublished, SubscriptionStateChanged};
use std::sync::Arc;

use crate::traits::{Publish, author_service::AuthorService};
use crate::utils::telegram_send_message_request::TelegramSendMessageRequest;

pub fn create_telegram_user_updates_service(
    bot_token: String,
    site_url: String,
    author_service: Arc<dyn AuthorService>,
) -> Result<
    Arc<impl Publish<SubscriptionStateChanged> + Publish<NewPostPublished>>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    if bot_token.is_empty() {
        return Err("bot token is empty".into());
    }
    Ok(Arc::new(TelegramUserUpdatesService {
        telegram_send_message_request: TelegramSendMessageRequest { bot_token },
        site_url,
        author_service,
    }))
}

struct TelegramUserUpdatesService {
    telegram_send_message_request: TelegramSendMessageRequest,
    site_url: String,
    author_service: Arc<dyn AuthorService>,
}

#[async_trait]
impl Publish<SubscriptionStateChanged> for TelegramUserUpdatesService {
    async fn publish(&self, event: SubscriptionStateChanged) {
        let message = if event.new_state == 1 {
            "Вы подписались на уведомления"
        } else {
            "Вы отписались от уведомлений"
        };
        self.telegram_send_message_request
            .send(&(event.user_telegram_id as i64), message)
            .await
    }
}

#[async_trait]
impl Publish<NewPostPublished> for TelegramUserUpdatesService {
    async fn publish(&self, event: NewPostPublished) {
        let Ok(telegram_ids) = self.author_service.subscribed_telegram_ids().await else {
            return;
        };
        for author_telegram_id in telegram_ids {
            self.telegram_send_message_request
                .send(
                    &(author_telegram_id as i64),
                    &event.absolute_url(&self.site_url),
                )
                .await;
        }
    }
}
