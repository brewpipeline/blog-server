use blog_generic::events::NewPostPublished;
use serde_json::json;
use std::sync::Arc;

use crate::traits::Publish;

pub fn create_discord_new_post_published_service(
    webhook_id: &'static str,
    webhook_token: &'static str,
    username: &'static str,
    avatar_url: &'static str,
    site_url: &'static str,
) -> Result<Arc<impl Publish<NewPostPublished>>, Box<dyn std::error::Error + Send + Sync>> {
    if webhook_id.is_empty() {
        return Err("webhook id is empty".into());
    }
    if webhook_token.is_empty() {
        return Err("webhook token is empty".into());
    }
    Ok(Arc::new(DiscordNewPostPublishedService {
        webhook_id,
        webhook_token,
        username,
        avatar_url,
        site_url,
    }))
}

struct DiscordNewPostPublishedService {
    webhook_id: &'static str,
    webhook_token: &'static str,
    username: &'static str,
    avatar_url: &'static str,
    site_url: &'static str,
}

impl DiscordNewPostPublishedService {
    async fn send(&self, content: &str) {
        let _ = reqwest::Client::new()
            .post(format!(
                "https://discord.com/api/webhooks/{ID}/{TOKEN}",
                ID = self.webhook_id,
                TOKEN = self.webhook_token,
            ))
            .json(&json!({
                "avatar_url": self.avatar_url,
                "username": self.username,
                "content": content,
            }))
            .send()
            .await;
    }
}

#[async_trait]
impl Publish<NewPostPublished> for DiscordNewPostPublishedService {
    async fn publish(&self, event: NewPostPublished) {
        self.send(&format!(
            "{SITE_URL}{PATH}",
            SITE_URL = self.site_url,
            PATH = event.post_sub_url
        ))
        .await;
    }
}
