use blog_generic::events::NewPostPublished;
use std::sync::Arc;

use crate::traits::Publish;
use crate::utils::discord_webhook_request::DiscordWebhookRequest;
use crate::utils::message_sink::MessageSink;
use crate::utils::telegram_send_message_request::TelegramChat;

pub fn create_telegram_new_post_published_service(
    bot_token: String,
    site_url: String,
    chat_id: i64,
) -> Result<Arc<impl Publish<NewPostPublished>>, Box<dyn std::error::Error + Send + Sync>> {
    if bot_token.is_empty() {
        return Err("bot token is empty".into());
    }
    Ok(Arc::new(WebhookNewPostPublishedService {
        sink: TelegramChat::new(bot_token, chat_id),
        site_url,
    }))
}

pub fn create_discord_new_post_published_service(
    webhook_id: String,
    webhook_token: String,
    username: String,
    avatar_url: String,
    site_url: String,
) -> Result<Arc<impl Publish<NewPostPublished>>, Box<dyn std::error::Error + Send + Sync>> {
    if webhook_id.is_empty() {
        return Err("webhook id is empty".into());
    }
    if webhook_token.is_empty() {
        return Err("webhook token is empty".into());
    }
    Ok(Arc::new(WebhookNewPostPublishedService {
        sink: DiscordWebhookRequest {
            webhook_id,
            webhook_token,
            username,
            avatar_url,
        },
        site_url,
    }))
}

struct WebhookNewPostPublishedService<Sink> {
    sink: Sink,
    site_url: String,
}

#[async_trait]
impl<Sink> Publish<NewPostPublished> for WebhookNewPostPublishedService<Sink>
where
    Sink: MessageSink,
{
    async fn publish(&self, event: NewPostPublished) {
        self.sink.send(&event.absolute_url(&self.site_url)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct Recorder(Mutex<Vec<String>>);

    #[async_trait]
    impl MessageSink for Recorder {
        async fn send(&self, message: &str) {
            self.0.lock().unwrap().push(message.to_owned());
        }
    }

    #[tokio::test]
    async fn a_published_post_is_sent_as_an_absolute_url() {
        let service = WebhookNewPostPublishedService {
            sink: Recorder::default(),
            site_url: "https://example.com".to_owned(),
        };

        service
            .publish(NewPostPublished {
                blog_user_id: 1,
                post_sub_url: "/post/a-slug/7".to_owned(),
            })
            .await;

        assert_eq!(
            service.sink.0.lock().unwrap().as_slice(),
            ["https://example.com/post/a-slug/7"]
        );
    }
}
