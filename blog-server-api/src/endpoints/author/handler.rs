use std::sync::Arc;

use blog_generic::entities::AuthorContainer;
use blog_server_services::traits::author_service::AuthorService;

use super::request_content::AuthorRequestContent;
use super::response_content_failure::AuthorResponseContentFailure;
use super::response_content_failure::AuthorResponseContentFailure::*;
use super::response_content_success::AuthorResponseContentSuccess;

pub async fn http_handler(
    (AuthorRequestContent {
        slug,
        author_service,
    },): (AuthorRequestContent,),
) -> Result<AuthorResponseContentSuccess, AuthorResponseContentFailure> {
    if slug.is_empty() {
        return Err(SlugEmpty);
    }

    let author = author_service
        .author_by_slug(&slug)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    Ok(author.into())
}

pub async fn direct_handler(
    slug: String,
    author_service: Arc<dyn AuthorService>,
) -> Option<AuthorContainer> {
    http_handler((AuthorRequestContent {
        slug,
        author_service,
    },))
    .await
    .ok()
    .map(|s| s.container)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use blog_server_services::traits::author_service::{
        Author, BaseAuthor, BaseMinimalAuthor, BaseSecondaryAuthor,
    };
    use screw_components::dyn_result::DResult;

    enum MockBehavior {
        Success(Option<Author>),
        Error,
    }

    struct MockAuthorService {
        behavior: MockBehavior,
    }

    #[async_trait]
    impl AuthorService for MockAuthorService {
        async fn authors_count_by_query(&self, _query: &String) -> DResult<u64> {
            unimplemented!()
        }

        async fn authors_by_query(
            &self,
            _query: &String,
            _offset: &u64,
            _limit: &u64,
        ) -> DResult<Vec<Author>> {
            unimplemented!()
        }

        async fn authors_count(&self) -> DResult<u64> {
            unimplemented!()
        }

        async fn authors(&self, _offset: &u64, _limit: &u64) -> DResult<Vec<Author>> {
            unimplemented!()
        }

        async fn authors_by_ids(
            &self,
            _ids: &std::collections::HashSet<u64>,
        ) -> DResult<Vec<Author>> {
            unimplemented!()
        }

        async fn author_by_id(&self, _id: &u64) -> DResult<Option<Author>> {
            unimplemented!()
        }

        async fn author_by_slug(&self, _slug: &String) -> DResult<Option<Author>> {
            match &self.behavior {
                MockBehavior::Success(author) => Ok(author.clone()),
                MockBehavior::Error => Err("db error".into()),
            }
        }

        async fn author_by_yandex_id(&self, _yandex_id: &u64) -> DResult<Option<Author>> {
            unimplemented!()
        }

        async fn author_by_telegram_id(&self, _telegram_id: &u64) -> DResult<Option<Author>> {
            unimplemented!()
        }

        async fn set_author_override_social_data_by_id(
            &self,
            _id: &u64,
            _override_social_data: &u8,
        ) -> DResult<()> {
            unimplemented!()
        }

        async fn update_minimal_custom_author_by_id(
            &self,
            _id: &u64,
            _base_minimal_author: &BaseMinimalAuthor,
        ) -> DResult<u64> {
            unimplemented!()
        }

        async fn update_minimal_social_author_by_id(
            &self,
            _id: &u64,
            _base_minimal_author: &BaseMinimalAuthor,
            _yandex_id: Option<&u64>,
            _telegram_id: Option<&u64>,
        ) -> DResult<u64> {
            unimplemented!()
        }

        async fn insert_minimal_social_author(
            &self,
            _base_minimal_author: &BaseMinimalAuthor,
            _yandex_id: Option<&u64>,
            _telegram_id: Option<&u64>,
        ) -> DResult<u64> {
            unimplemented!()
        }

        async fn update_secondary_author_by_id(
            &self,
            _id: &u64,
            _base_secondary_author: &BaseSecondaryAuthor,
        ) -> DResult<u64> {
            unimplemented!()
        }

        async fn set_author_blocked_by_id(&self, _id: &u64, _is_blocked: &u8) -> DResult<()> {
            unimplemented!()
        }

        async fn set_author_subscription_by_id(
            &self,
            _id: &u64,
            _is_subscribed: &u8,
        ) -> DResult<()> {
            unimplemented!()
        }
    }

    fn sample_author() -> Author {
        Author {
            id: 1,
            base: BaseAuthor {
                slug: "john".into(),
                first_name: Some("John".into()),
                middle_name: None,
                last_name: Some("Doe".into()),
                mobile: None,
                email: None,
                password_hash: None,
                registered_at: 0,
                status: None,
                image_url: None,
                editor: 0,
                blocked: 0,
                yandex_id: None,
                telegram_id: None,
                notification_subscribed: None,
                override_social_data: 0,
            },
        }
    }

    #[tokio::test]
    async fn empty_slug_returns_error() {
        let service = Arc::new(MockAuthorService {
            behavior: MockBehavior::Success(None),
        });
        let result = http_handler((AuthorRequestContent {
            slug: String::new(),
            author_service: service,
        },))
        .await;
        assert!(matches!(result, Err(SlugEmpty)));
    }

    #[tokio::test]
    async fn not_found_returns_error() {
        let service = Arc::new(MockAuthorService {
            behavior: MockBehavior::Success(None),
        });
        let result = http_handler((AuthorRequestContent {
            slug: "missing".into(),
            author_service: service,
        },))
        .await;
        assert!(matches!(result, Err(NotFound)));
    }

    #[tokio::test]
    async fn db_error_propagates() {
        let service = Arc::new(MockAuthorService {
            behavior: MockBehavior::Error,
        });
        let result = http_handler((AuthorRequestContent {
            slug: "john".into(),
            author_service: service,
        },))
        .await;
        assert!(matches!(result, Err(DatabaseError { .. })));
    }

    #[tokio::test]
    async fn success_returns_author() {
        let service = Arc::new(MockAuthorService {
            behavior: MockBehavior::Success(Some(sample_author())),
        });
        let result = http_handler((AuthorRequestContent {
            slug: "john".into(),
            author_service: service,
        },))
        .await;
        assert!(matches!(result, Ok(_)));
    }
}
