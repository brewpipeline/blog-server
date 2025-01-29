use blog_generic::events::SubscriptionStateChanged;
use rbatis::rbdc::rt::tokio;
use screw_components::dyn_result::{DError, DResult};
use std::sync::Arc;

use crate::traits::author_service::{Author, AuthorService, BaseMinimalAuthor};
use crate::traits::social_service::{SocialId, SocialService as SocialServiceTrait};
use crate::traits::Publish;

pub fn create_social_service(
    author_service: Arc<dyn AuthorService>,
    subscription_state_changed_service: Arc<dyn Publish<SubscriptionStateChanged>>,
) -> Arc<dyn SocialServiceTrait> {
    Arc::new(SocialService {
        author_service,
        subscription_state_changed_service,
    })
}
struct SocialService {
    author_service: Arc<dyn AuthorService>,
    subscription_state_changed_service: Arc<dyn Publish<SubscriptionStateChanged>>,
}

#[async_trait]
impl SocialServiceTrait for SocialService {
    async fn set_subscribe_for_author(&self, author: &Author, subscribe: &u8) -> DResult<()> {
        if let Some(telegram_id) = author.base.telegram_id {
            let event = SubscriptionStateChanged {
                blog_user_id: author.id,
                user_telegram_id: telegram_id,
                new_state: *subscribe,
            };

            let subscription_state_changed_service =
                self.subscription_state_changed_service.clone();
            tokio::spawn(async move { subscription_state_changed_service.publish(event).await });
        } else {
            Err(DError::from("not supported for current author"))?
        }

        self.author_service
            .set_author_subscription_by_id(&author.id, &subscribe)
            .await
    }
    async fn process_auth_by_id(
        &self,
        social_id: &SocialId,
        base_minimal_author: &BaseMinimalAuthor,
    ) -> DResult<Author> {
        let (social_author_id, _is_new_author) = if let Some(author) = match &social_id {
            SocialId::TelegramId(telegram_id) => {
                self.author_service
                    .author_by_telegram_id(&telegram_id)
                    .await?
            }
            SocialId::YandexId(yandex_id) => {
                self.author_service.author_by_yandex_id(&yandex_id).await?
            }
        } {
            (
                if author.base.override_social_data != 0 {
                    author.id
                } else {
                    let updated_id = self
                        .author_service
                        .update_minimal_social_author_by_id(
                            &author.id,
                            base_minimal_author,
                            social_id.yandex_id(),
                            social_id.telegram_id(),
                        )
                        .await?;
                    updated_id
                },
                false,
            )
        } else {
            let insert_id = self
                .author_service
                .insert_minimal_social_author(
                    base_minimal_author,
                    social_id.yandex_id(),
                    social_id.telegram_id(),
                )
                .await?;
            (insert_id, true)
        };

        let social_author = self
            .author_service
            .author_by_id(&social_author_id)
            .await?
            .ok_or::<DError>("insert error".into())?;

        Ok(social_author)
    }
}
