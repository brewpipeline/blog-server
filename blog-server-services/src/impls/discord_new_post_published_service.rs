use blog_generic::events::NewPostPublished;
use std::sync::Arc;

use crate::traits::Publish;
use crate::utils::discord_webhook_request::DiscordWebhookRequest;

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
    Ok(Arc::new(DiscordNewPostPublishedService {
        discord_webhook_request: DiscordWebhookRequest {
            webhook_id,
            webhook_token,
            username,
            avatar_url,
        },
        site_url,
    }))
}

struct DiscordNewPostPublishedService {
    discord_webhook_request: DiscordWebhookRequest,
    site_url: String,
}

#[async_trait]
impl Publish<NewPostPublished> for DiscordNewPostPublishedService {
    async fn publish(&self, event: NewPostPublished) {
        self.discord_webhook_request
            .send(&format!(
                "{SITE_URL}{PATH}",
                SITE_URL = self.site_url,
                PATH = event.post_sub_url
            ))
            .await;
    }
}
