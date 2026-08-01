use blog_generic::events::{NewPostPublished, SubscriptionStateChanged};
use blog_server_services::impls::{
    create_discord_new_post_published_service, create_entity_comment_service,
    create_entity_post_service, create_rbatis_author_service, create_rbatis_comment_service,
    create_rbatis_post_service, create_social_service, create_telegram_new_post_published_service,
    create_telegram_user_updates_service,
};
use blog_server_services::traits::author_service::AuthorService;
use blog_server_services::traits::comment_service::CommentService;
use blog_server_services::traits::entity_comment_service::EntityCommentService;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::PostService;
use blog_server_services::traits::social_service::SocialService;
use blog_server_services::traits::{Publish, PublishCollection};
use config::Config;
use rbatis::rbatis::RBatis;
use serde::Deserialize;
use std::sync::Arc;

pub trait Resolve<T>: Send + Sync {
    fn resolve(&self) -> T;
}

macro_rules! extensions {
    ($($field:ident: $service:ty,)*) => {
        pub trait ExtensionsProviderType: $(Resolve<Arc<$service>> +)* {}

        struct ExtensionsProvider {
            $($field: Arc<$service>,)*
        }

        impl ExtensionsProviderType for ExtensionsProvider {}

        $(
            impl Resolve<Arc<$service>> for ExtensionsProvider {
                fn resolve(&self) -> Arc<$service> {
                    self.$field.clone()
                }
            }
        )*
    };
}

extensions! {
    author_service: dyn AuthorService,
    post_service: dyn PostService,
    comment_service: dyn CommentService,
    entity_comment_service: dyn EntityCommentService,
    entity_post_service: dyn EntityPostService,
    new_post_published_service: dyn Publish<NewPostPublished>,
    subscription_state_changed_service: dyn Publish<SubscriptionStateChanged>,
    social_service: dyn SocialService,
}

pub fn make_extensions<U>(
    config: Config,
    rbatis: RBatis,
    updates_service: Option<Arc<U>>,
) -> impl ExtensionsProviderType
where
    U: Publish<NewPostPublished> + Publish<SubscriptionStateChanged> + 'static,
{
    let author_service = create_rbatis_author_service(rbatis.clone());

    let new_post_published_service: Arc<dyn Publish<NewPostPublished>>;
    let subscription_state_changed_service: Arc<dyn Publish<SubscriptionStateChanged>>;
    match updates_service {
        Some(updates_service) => {
            new_post_published_service = updates_service.clone();
            subscription_state_changed_service = updates_service.clone();
        }
        None => match create_telegram_user_updates_service(
            crate::TELEGRAM_BOT_TOKEN.to_string(),
            crate::SITE_URL.to_string(),
            author_service.clone(),
        ) {
            Ok(telegram_user_updates_service) => {
                println!("Alternative Telegram updates service used");
                new_post_published_service = telegram_user_updates_service.clone();
                subscription_state_changed_service = telegram_user_updates_service.clone();
            }
            Err(err) => {
                println!("Failed to create an alternative Telegram updates service: {err}");
                let empty_updates_service = Arc::new(EmptyUpdatesService);
                new_post_published_service = empty_updates_service.clone();
                subscription_state_changed_service = empty_updates_service.clone();
            }
        },
    }

    let new_post_published_service: Arc<dyn Publish<NewPostPublished>> = {
        let telegram_chat_ids: Vec<i64> = config
            .get("telegram_chat_ids")
            .expect("Failed to retrieve 'telegram_chat_ids' from config");

        let telegram_services: Vec<Arc<dyn Publish<NewPostPublished>>> = telegram_chat_ids
            .into_iter()
            .map(|id| {
                create_telegram_new_post_published_service(
                    crate::TELEGRAM_BOT_TOKEN.to_string(),
                    crate::SITE_URL.to_string(),
                    id,
                )
                .expect("Failed to create Telegram NewPostPublished service")
                    as Arc<dyn Publish<NewPostPublished>>
            })
            .collect();

        #[derive(Deserialize)]
        struct DiscordWebhook {
            webhook_id: String,
            webhook_token: String,
            username: String,
            avatar_url: String,
        }
        let discord_webhooks: Vec<DiscordWebhook> = config
            .get("discord_webhooks")
            .expect("Failed to retrieve 'discord_webhooks' from config");

        let discord_services: Vec<Arc<dyn Publish<NewPostPublished>>> = discord_webhooks
            .into_iter()
            .map(|w| {
                create_discord_new_post_published_service(
                    w.webhook_id,
                    w.webhook_token,
                    w.username,
                    w.avatar_url,
                    crate::SITE_URL.to_string(),
                )
                .expect("Failed to create Discord NewPostPublished service")
                    as Arc<dyn Publish<NewPostPublished>>
            })
            .collect();

        Arc::new(PublishCollection::new(
            std::iter::once(new_post_published_service)
                .chain(telegram_services.into_iter())
                .chain(discord_services.into_iter())
                .collect(),
        ))
    };

    ExtensionsProvider {
        author_service: author_service.clone(),
        post_service: create_rbatis_post_service(rbatis.clone()),
        comment_service: create_rbatis_comment_service(rbatis.clone()),
        entity_comment_service: create_entity_comment_service(author_service.clone()),
        entity_post_service: create_entity_post_service(author_service.clone()),
        new_post_published_service: new_post_published_service.clone(),
        subscription_state_changed_service: subscription_state_changed_service.clone(),
        social_service: create_social_service(
            author_service.clone(),
            subscription_state_changed_service.clone(),
        ),
    }
}

struct EmptyUpdatesService;

#[async_trait]
impl Publish<SubscriptionStateChanged> for EmptyUpdatesService {
    async fn publish(&self, _: SubscriptionStateChanged) {}
}

#[async_trait]
impl Publish<NewPostPublished> for EmptyUpdatesService {
    async fn publish(&self, _: NewPostPublished) {}
}
