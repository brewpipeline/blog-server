use rbatis::rbatis::RBatis;
use screw_components::dyn_result::DResult;
use std::collections::HashSet;

use crate::traits::author_service::{
    Author, AuthorService, BaseMinimalAuthor, BaseSecondaryAuthor,
};
use crate::utils::time_utils;

pub fn create_rbatis_author_service(rb: RBatis) -> Box<dyn AuthorService> {
    Box::new(RbatisAuthorService { rb })
}

impl_select!(Author {select_by_id(id: &u64) -> Option => 
    "`WHERE id = #{id} LIMIT 1`"});
impl_select!(Author {select_by_yandex_id(yandex_id: &u64) -> Option =>
    "`WHERE yandex_id = #{yandex_id} LIMIT 1`"});
impl_select!(Author {select_by_telegram_id(telegram_id: &u64) -> Option =>
    "`WHERE telegram_id = #{telegram_id} LIMIT 1`"});
impl_select!(Author {select_by_slug(slug: &String) -> Option => 
    "`WHERE slug = #{slug} LIMIT 1`"});
impl_select!(Author {select_all_with_offset_and_limit(offset: &u64, limit: &u64) => 
    "`LIMIT #{limit} OFFSET #{offset}`"});
impl_select!(Author {select_all_by_query_with_offset_and_limit(query: &String, offset: &u64, limit: &u64) => 
    "`WHERE author.slug ILIKE '%' || #{query} || '%' OR author.first_name ILIKE '%' || #{query} || '%' OR author.middle_name ILIKE '%' || #{query} || '%' OR author.last_name ILIKE '%' || #{query} || '%' LIMIT #{limit} OFFSET #{offset}`"});

impl Author {
    #[py_sql(
        "
        SELECT COUNT(1) \
        FROM author \
    "
    )]
    async fn count(rb: &RBatis) -> rbatis::Result<u64> {
        impled!()
    }
    #[py_sql(
        "
        SELECT COUNT(1) \
        FROM author \
        WHERE author.slug ILIKE '%' || #{query} || '%' OR author.first_name ILIKE '%' || #{query} || '%' OR author.middle_name ILIKE '%' || #{query} || '%' OR author.last_name ILIKE '%' || #{query} || '%' \
    "
    )]
    async fn count_by_query(rb: &RBatis, query: &String) -> rbatis::Result<u64> {
        impled!()
    }
    #[py_sql(
        "
        SELECT \
            author.* \
        FROM author \
        WHERE \
            author.id IN (
                trim ',': for _,id in ids:
                    #{id},
                ) \
    "
    )]
    async fn select_by_ids(rb: &RBatis, ids: &HashSet<u64>) -> rbatis::Result<Vec<Author>> {
        impled!()
    }
    #[py_sql(
        "
        UPDATE author \
        SET \
            blocked = #{is_blocked} \
        WHERE id = #{id}
    "
    )]
    async fn set_blocked_by_id(rb: &RBatis, id: &u64, is_blocked: &u8) -> rbatis::Result<()> {
        impled!()
    }
    #[py_sql(
        "
        UPDATE author \
        SET \
            notification_subscribed = #{is_subscribed} \
        WHERE id = #{id}
    "
    )]
    async fn set_notification_subscribed_by_id(
        rb: &RBatis,
        id: &u64,
        is_subscribed: &u8,
    ) -> rbatis::Result<()> {
        impled!()
    }
    #[py_sql(
        "
        UPDATE author \
        SET \
            override_social_data = #{override_social_data} \
        WHERE id = #{id}
    "
    )]
    async fn set_override_social_data_by_id(
        rb: &RBatis,
        id: &u64,
        override_social_data: &u8,
    ) -> rbatis::Result<()> {
        impled!()
    }
}

struct RbatisAuthorService {
    rb: RBatis,
}

impl RbatisAuthorService {
    #[py_sql(
        "
        INSERT INTO author (
            slug,
            first_name,
            last_name,
            registered_at,
            image_url,
            yandex_id,
            telegram_id,
            override_social_data
        ) VALUES (
            #{base_minimal_author.slug},
            #{base_minimal_author.first_name},
            #{base_minimal_author.last_name},
            to_timestamp(#{registered_at}),
            #{base_minimal_author.image_url},
            #{yandex_id},
            #{telegram_id},
            0
        ) RETURNING id
    "
    )]
    async fn insert_minimal_social_author(
        rb: &RBatis,
        base_minimal_author: &BaseMinimalAuthor,
        registered_at: &u64,
        yandex_id: Option<&u64>,
        telegram_id: Option<&u64>,
    ) -> rbatis::Result<u64> {
        impled!()
    }

    #[py_sql(
        "
        UPDATE author \
        SET \
            slug = #{base_minimal_author.slug}, \
            first_name = #{base_minimal_author.first_name}, \
            last_name = #{base_minimal_author.last_name}, \
            image_url = #{base_minimal_author.image_url}, \
            yandex_id = #{yandex_id}, \
            telegram_id = #{telegram_id}, \
            override_social_data = 0 \
        WHERE id = #{author_id} \
        RETURNING id
    "
    )]
    async fn update_minimal_social_author_by_id(
        rb: &RBatis,
        author_id: &u64,
        base_minimal_author: &BaseMinimalAuthor,
        yandex_id: Option<&u64>,
        telegram_id: Option<&u64>,
    ) -> rbatis::Result<u64> {
        impled!()
    }

    #[py_sql(
        "
        UPDATE author \
        SET \
            slug = #{base_minimal_author.slug}, \
            first_name = #{base_minimal_author.first_name}, \
            last_name = #{base_minimal_author.last_name}, \
            image_url = #{base_minimal_author.image_url}, \
            override_social_data = 1 \
        WHERE id = #{author_id} \
        RETURNING id
    "
    )]
    async fn update_minimal_custom_author_by_id(
        rb: &RBatis,
        author_id: &u64,
        base_minimal_author: &BaseMinimalAuthor,
    ) -> rbatis::Result<u64> {
        impled!()
    }

    #[py_sql(
        "
        UPDATE author \
        SET \
            email = #{base_secondary_author.email}, \
            mobile = #{base_secondary_author.mobile}, \
            status = #{base_secondary_author.status} \
        WHERE id = #{author_id} \
        RETURNING id
    "
    )]
    async fn update_secondary_author_by_id(
        rb: &RBatis,
        author_id: &u64,
        base_secondary_author: &BaseSecondaryAuthor,
    ) -> rbatis::Result<u64> {
        impled!()
    }
}

#[async_trait]
impl AuthorService for RbatisAuthorService {
    async fn authors_count_by_query(&self, query: &String) -> DResult<u64> {
        Ok(Author::count_by_query(&self.rb, query).await?)
    }
    async fn authors_by_query(
        &self,
        query: &String,
        offset: &u64,
        limit: &u64,
    ) -> DResult<Vec<Author>> {
        Ok(Author::select_all_by_query_with_offset_and_limit(
            &mut self.rb.clone(),
            query,
            offset,
            limit,
        )
        .await?)
    }
    async fn authors_count(&self) -> DResult<u64> {
        Ok(Author::count(&self.rb).await?)
    }
    async fn authors(&self, offset: &u64, limit: &u64) -> DResult<Vec<Author>> {
        Ok(Author::select_all_with_offset_and_limit(&mut self.rb.clone(), offset, limit).await?)
    }
    async fn authors_by_ids(&self, ids: &HashSet<u64>) -> DResult<Vec<Author>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(Author::select_by_ids(&self.rb, ids).await?)
    }
    async fn author_by_id(&self, id: &u64) -> DResult<Option<Author>> {
        Ok(Author::select_by_id(&mut self.rb.clone(), id).await?)
    }
    async fn author_by_slug(&self, slug: &String) -> DResult<Option<Author>> {
        Ok(Author::select_by_slug(&mut self.rb.clone(), slug).await?)
    }
    async fn author_by_yandex_id(&self, yandex_id: &u64) -> DResult<Option<Author>> {
        Ok(Author::select_by_yandex_id(&mut self.rb.clone(), yandex_id).await?)
    }
    async fn author_by_telegram_id(&self, telegram_id: &u64) -> DResult<Option<Author>> {
        Ok(Author::select_by_telegram_id(&mut self.rb.clone(), telegram_id).await?)
    }
    async fn set_author_override_social_data_by_id(
        &self,
        id: &u64,
        override_social_data: &u8,
    ) -> DResult<()> {
        let _ =
            Author::set_override_social_data_by_id(&mut self.rb.clone(), &id, override_social_data)
                .await;
        Ok(())
    }
    async fn update_minimal_custom_author_by_id(
        &self,
        id: &u64,
        base_minimal_author: &BaseMinimalAuthor,
    ) -> DResult<u64> {
        let updated_id = RbatisAuthorService::update_minimal_custom_author_by_id(
            &mut self.rb.clone(),
            &id,
            base_minimal_author,
        )
        .await?;
        Ok(updated_id)
    }
    async fn update_minimal_social_author_by_id(
        &self,
        id: &u64,
        base_minimal_author: &BaseMinimalAuthor,
        yandex_id: Option<&u64>,
        telegram_id: Option<&u64>,
    ) -> DResult<u64> {
        let updated_id = RbatisAuthorService::update_minimal_social_author_by_id(
            &mut self.rb.clone(),
            &id,
            base_minimal_author,
            yandex_id,
            telegram_id,
        )
        .await?;
        Ok(updated_id)
    }
    async fn insert_minimal_social_author(
        &self,
        base_minimal_author: &BaseMinimalAuthor,
        yandex_id: Option<&u64>,
        telegram_id: Option<&u64>,
    ) -> DResult<u64> {
        let inserted_id = RbatisAuthorService::insert_minimal_social_author(
            &mut self.rb.clone(),
            base_minimal_author,
            &time_utils::now_as_secs(),
            yandex_id,
            telegram_id,
        )
        .await?;
        Ok(inserted_id)
    }
    async fn update_secondary_author_by_id(
        &self,
        id: &u64,
        base_secondary_author: &BaseSecondaryAuthor,
    ) -> DResult<u64> {
        let updated_id = RbatisAuthorService::update_secondary_author_by_id(
            &mut self.rb.clone(),
            id,
            base_secondary_author,
        )
        .await?;
        Ok(updated_id)
    }
    async fn set_author_blocked_by_id(&self, id: &u64, is_blocked: &u8) -> DResult<()> {
        let _ = Author::set_blocked_by_id(&mut self.rb.clone(), &id, &is_blocked).await;
        Ok(())
    }
    async fn set_author_subscription_by_id(&self, id: &u64, is_subscribed: &u8) -> DResult<()> {
        let _ =
            Author::set_notification_subscribed_by_id(&mut self.rb.clone(), &id, &is_subscribed)
                .await;
        Ok(())
    }
}
