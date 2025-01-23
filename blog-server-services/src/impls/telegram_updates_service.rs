use blog_generic::events::{NewPostPublished, SubscriptionStateChanged};
use serde_json::json;
use std::sync::Arc;

use crate::traits::{author_service::AuthorService, Publish};

pub fn create_telegram_updates_service(
    bot_token: &'static str,
    site_url: &'static str,
    author_service: Arc<dyn AuthorService>,
) -> Result<
    Arc<impl Publish<SubscriptionStateChanged> + Publish<NewPostPublished>>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    if bot_token.is_empty() {
        return Err("bot token is empty".into());
    }
    Ok(Arc::new(TelegramUpdatesService {
        bot_token,
        site_url,
        author_service,
    }))
}

struct TelegramUpdatesService {
    bot_token: &'static str,
    site_url: &'static str,
    author_service: Arc<dyn AuthorService>,
}

impl TelegramUpdatesService {
    async fn send(&self, chat_id: &u64, text: &str) {
        let _ = reqwest::Client::new()
            .post(format!(
                "https://api.telegram.org/bot{TOKEN}/sendMessage",
                TOKEN = self.bot_token
            ))
            .json(&json!({
                "chat_id": chat_id,
                "text":  text,
            }))
            .send()
            .await;
    }
}

#[async_trait]
impl Publish<SubscriptionStateChanged> for TelegramUpdatesService {
    async fn publish(&self, event: SubscriptionStateChanged) {
        let message = if event.new_state == 1 {
            "Вы подписались на уведомления"
        } else {
            "Вы отписались от уведомлений"
        };
        self.send(&event.user_telegram_id, message).await
    }
}

#[async_trait]
impl Publish<NewPostPublished> for TelegramUpdatesService {
    async fn publish(&self, event: NewPostPublished) {
        let Ok(authors) = self.author_service.authors(&0, &u64::MAX).await else {
            return;
        };
        for author in authors {
            let Some(author_telegram_id) = author.base.telegram_id else {
                continue;
            };
            self.send(
                &author_telegram_id,
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
