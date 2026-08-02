use screw_components::dyn_result::{DError, DResult};

use super::author_service::*;

pub enum SubscribeRejection {
    NoNotificationChannel,
    Other(DError),
}

impl From<DError> for SubscribeRejection {
    fn from(error: DError) -> Self {
        SubscribeRejection::Other(error)
    }
}

pub enum SocialId {
    TelegramId(u64),
    YandexId(u64),
}

impl SocialId {
    pub fn telegram_id(&self) -> Option<&u64> {
        match self {
            SocialId::TelegramId(telegram_id) => Some(telegram_id),
            SocialId::YandexId(_) => None,
        }
    }
    pub fn yandex_id(&self) -> Option<&u64> {
        match self {
            SocialId::TelegramId(_) => None,
            SocialId::YandexId(yandex_id) => Some(yandex_id),
        }
    }
}

#[async_trait]
pub trait SocialService: Send + Sync {
    async fn set_subscribe_for_author(
        &self,
        author: &Author,
        subscribe: &u8,
    ) -> Result<(), SubscribeRejection>;
    async fn process_auth_by_id(
        &self,
        social_id: &SocialId,
        base_minimal_author: &BaseMinimalAuthor,
    ) -> DResult<Author>;
}
