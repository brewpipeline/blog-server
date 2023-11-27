use blog_generic::events::SubscriptionStateChanged;
use rbatis::rbdc::rt::tokio;
use screw_components::dyn_result::{DError, DResult};
use std::sync::Arc;

use crate::traits::author_service::{Author, AuthorService, BaseMinimalAuthor};
use crate::traits::event_bus_service::EventBusService;
use crate::traits::social_service::{SocialId, SocialService as SocialServiceTrait};

pub fn create_social_service(
    author_service: Arc<Box<dyn AuthorService>>,
    event_bus_service: Arc<Box<dyn EventBusService>>,
) -> Box<dyn SocialServiceTrait> {
    Box::new(SocialService {
        author_service,
        event_bus_service,
    })
}
struct SocialService {
    author_service: Arc<Box<dyn AuthorService>>,
    event_bus_service: Arc<Box<dyn EventBusService>>,
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

            let event_bus_service = self.event_bus_service.clone();
            tokio::spawn(async move { event_bus_service.publish(event).await });
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
        let (social_author_id, is_new_author) = if let Some(author) = match &social_id {
            SocialId::TelegramId(telegram_id) => {
                self.author_service
                    .author_by_telegram_id(&telegram_id)
                    .await?
            }
            SocialId::YandexId(yandex_id) => {
                self.author_service.author_by_yandex_id(&yandex_id).await?
            }
        } {
            (if author.base.override_social_data != 0 {
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
            }, false)
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

        if is_new_author {
            let _ = self.set_subscribe_for_author(&social_author, &1).await;
        }

        Ok(social_author)
    }
}
