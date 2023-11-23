use std::collections::HashSet;

use blog_generic::entities::{
    Author as EAuthor, CommonMinimalAuthor as ECommonMinimalAuthor,
    CommonSecondaryAuthor as ECommonSecondaryAuthor,
};
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseAuthor {
    pub slug: String,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub registered_at: u64,
    pub status: Option<String>,
    pub image_url: Option<String>,
    pub editor: u8,
    pub blocked: u8,
    pub yandex_id: Option<u64>,
    pub telegram_id: Option<u64>,
    pub notification_subscribed: Option<u8>,
    pub override_social_data: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseMinimalAuthor {
    pub slug: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image_url: Option<String>,
}

impl From<ECommonMinimalAuthor> for BaseMinimalAuthor {
    fn from(value: ECommonMinimalAuthor) -> Self {
        BaseMinimalAuthor {
            slug: value.slug,
            first_name: value.first_name,
            last_name: value.last_name,
            image_url: value.image_url,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BaseSecondaryAuthor {
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub status: Option<String>,
}

impl From<ECommonSecondaryAuthor> for BaseSecondaryAuthor {
    fn from(value: ECommonSecondaryAuthor) -> Self {
        BaseSecondaryAuthor {
            email: value.email,
            mobile: value.mobile,
            status: value.status,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Author {
    pub id: u64,
    #[serde(flatten)]
    pub base: BaseAuthor,
}

impl Into<EAuthor> for Author {
    fn into(self) -> EAuthor {
        EAuthor {
            id: self.id,
            slug: self.base.slug,
            first_name: self.base.first_name.filter(|_| self.base.blocked == 0),
            last_name: self.base.last_name.filter(|_| self.base.blocked == 0),
            middle_name: self.base.middle_name.filter(|_| self.base.blocked == 0),
            mobile: self.base.mobile.filter(|_| self.base.blocked == 0),
            email: self.base.email.filter(|_| self.base.blocked == 0),
            registered_at: self.base.registered_at,
            status: self.base.status.filter(|_| self.base.blocked == 0),
            image_url: self.base.image_url.filter(|_| self.base.blocked == 0),
            editor: self.base.editor,
            blocked: self.base.blocked,
            notification_subscribed: self.base.notification_subscribed,
            override_social_data: self.base.override_social_data,
        }
    }
}

#[async_trait]
pub trait AuthorService: Send + Sync {
    async fn authors_count_by_query(&self, query: &String) -> DResult<u64>;
    async fn authors_by_query(
        &self,
        query: &String,
        offset: &u64,
        limit: &u64,
    ) -> DResult<Vec<Author>>;
    async fn authors_count(&self) -> DResult<u64>;
    async fn authors(&self, offset: &u64, limit: &u64) -> DResult<Vec<Author>>;
    async fn authors_by_ids(&self, ids: &HashSet<u64>) -> DResult<Vec<Author>>;
    async fn author_by_id(&self, id: &u64) -> DResult<Option<Author>>;
    async fn author_by_slug(&self, slug: &String) -> DResult<Option<Author>>;
    async fn author_by_yandex_id(&self, yandex_id: &u64) -> DResult<Option<Author>>;
    async fn author_by_telegram_id(&self, telegram_id: &u64) -> DResult<Option<Author>>;
    async fn set_author_override_social_data_by_id(
        &self,
        id: &u64,
        override_social_data: &u8,
    ) -> DResult<()>;
    async fn update_minimal_custom_author_by_id(
        &self,
        id: &u64,
        base_minimal_author: &BaseMinimalAuthor,
    ) -> DResult<u64>;
    async fn update_minimal_social_author_by_id(
        &self,
        id: &u64,
        base_minimal_author: &BaseMinimalAuthor,
        yandex_id: Option<&u64>,
        telegram_id: Option<&u64>,
    ) -> DResult<u64>;
    async fn insert_minimal_social_author(
        &self,
        base_minimal_author: &BaseMinimalAuthor,
        yandex_id: Option<&u64>,
        telegram_id: Option<&u64>,
    ) -> DResult<u64>;
    async fn update_secondary_author_by_id(
        &self,
        id: &u64,
        base_secondary_author: &BaseSecondaryAuthor,
    ) -> DResult<u64>;
    async fn set_author_blocked_by_id(&self, id: &u64, is_blocked: &u8) -> DResult<()>;
    async fn set_author_subscription_by_id(&self, id: &u64, is_subscribed: &u8) -> DResult<()>;
}
