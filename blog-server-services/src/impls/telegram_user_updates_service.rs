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
        let Ok(authors) = self.author_service.authors(&0, &(i64::MAX as u64)).await else {
            return;
        };
        for author in authors {
            if author.base.notification_subscribed.unwrap_or_default() == 0 {
                continue;
            }
            let Some(author_telegram_id) = author.base.telegram_id else {
                continue;
            };
            self.telegram_send_message_request
                .send(
                    &(author_telegram_id as i64),
                    &format!(
                        "{SITE_URL}{PATH}",
                        SITE_URL = self.site_url,
                        PATH = event.post_sub_url
                    ),
                )
                .await;
        }
    }
}
