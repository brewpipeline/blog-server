use chrono::{DateTime, Days, Utc};
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use tokio::sync::Mutex;

use super::request_content::LoginRequestContent;
use super::response_content_failure::LoginResponseContentFailure;
use super::response_content_failure::LoginResponseContentFailure::*;
use super::response_content_success::LoginResponseContentSuccess;
use crate::utils::{auth, password};
use blog_generic::entities::LoginQuestion;
use password_hash::Error;

static LOGIN_TRY_STORAGE: Lazy<Mutex<HashMap<u64, HashSet<DateTime<Utc>>>>> =
    Lazy::new(|| Mutex::new(Default::default()));

pub async fn http_handler(
    (LoginRequestContent {
        login_question,
        author_service,
    },): (LoginRequestContent,),
) -> Result<LoginResponseContentSuccess, LoginResponseContentFailure> {
    let LoginQuestion { slug, password } = login_question.map_err(|e| ParamsDecodeError {
        reason: e.to_string(),
    })?;

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

    let mut login_try_storage = LOGIN_TRY_STORAGE.lock().await;
    let now = Utc::now();

    if let Some(login_try) = login_try_storage.remove(&author.id) {
        let yesterday = now - Days::new(1);
        let login_try_actual = login_try
            .into_iter()
            .filter(|t| t > &yesterday)
            .collect::<HashSet<_>>();
        let login_try_actual_len = login_try_actual.len();
        (*login_try_storage).insert(author.id, login_try_actual);
        if login_try_actual_len > 20 {
            return Err(Blocked);
        }
    }

    let Some(password_hash) = &author.base.password_hash else {
        return Err(WrongPassword);
    };

    password::verify(&password, password_hash).map_err(|e| {
        if let Some(login_try) = login_try_storage.get_mut(&author.id) {
            login_try.insert(now);
        } else {
            (*login_try_storage).insert(author.id, HashSet::from_iter(vec![now]));
        }
        match e {
            Error::Password => WrongPassword,
            _ => PasswordVerificationError {
                reason: e.to_string(),
            },
        }
    })?;

    let token = auth::token(author).map_err(|e| TokenGeneratingError {
        reason: e.to_string(),
    })?;

    Ok(token.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use blog_server_services::traits::author_service::{
        Author, BaseAuthor, BaseMinimalAuthor, BaseSecondaryAuthor, AuthorService,
    };
    use screw_components::dyn_result::DResult;
    use std::sync::Arc;

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

        async fn author_by_telegram_id(
            &self,
            _telegram_id: &u64,
        ) -> DResult<Option<Author>> {
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

    fn sample_author(password_hash: Option<String>) -> Author {
        Author {
            id: 1,
            base: BaseAuthor {
                slug: "john".into(),
                first_name: None,
                middle_name: None,
                last_name: None,
                mobile: None,
                email: None,
                password_hash,
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

    async fn reset_storage() {
        LOGIN_TRY_STORAGE.lock().await.clear();
    }

    #[tokio::test]
    async fn empty_slug_returns_error() {
        reset_storage().await;
        let service = Arc::new(MockAuthorService { behavior: MockBehavior::Success(None) });
        let result = http_handler((LoginRequestContent {
            login_question: Ok(LoginQuestion { slug: String::new(), password: String::new() }),
            author_service: service,
        },)).await;
        assert!(matches!(result, Err(SlugEmpty)));
    }

    #[tokio::test]
    async fn not_found_returns_error() {
        reset_storage().await;
        let service = Arc::new(MockAuthorService { behavior: MockBehavior::Success(None) });
        let result = http_handler((LoginRequestContent {
            login_question: Ok(LoginQuestion { slug: "missing".into(), password: "pwd".into() }),
            author_service: service,
        },)).await;
        assert!(matches!(result, Err(NotFound)));
    }

    #[tokio::test]
    async fn db_error_propagates() {
        reset_storage().await;
        let service = Arc::new(MockAuthorService { behavior: MockBehavior::Error });
        let result = http_handler((LoginRequestContent {
            login_question: Ok(LoginQuestion { slug: "john".into(), password: "pwd".into() }),
            author_service: service,
        },)).await;
        assert!(matches!(result, Err(DatabaseError { .. })));
    }

    #[tokio::test]
    async fn wrong_password_returns_error() {
        reset_storage().await;
        let hash = password::hash(&"secret".to_string()).unwrap();
        let service = Arc::new(MockAuthorService {
            behavior: MockBehavior::Success(Some(sample_author(Some(hash)))),
        });
        let result = http_handler((LoginRequestContent {
            login_question: Ok(LoginQuestion { slug: "john".into(), password: "wrong".into() }),
            author_service: service,
        },)).await;
        assert!(matches!(result, Err(WrongPassword)));
    }

    #[tokio::test]
    async fn success_returns_token() {
        reset_storage().await;
        std::env::set_var("JWT_SECRET", "secret");
        let hash = password::hash(&"secret".to_string()).unwrap();
        let service = Arc::new(MockAuthorService {
            behavior: MockBehavior::Success(Some(sample_author(Some(hash.clone())))),
        });
        let result = http_handler((LoginRequestContent {
            login_question: Ok(LoginQuestion { slug: "john".into(), password: "secret".into() }),
            author_service: service,
        },)).await;
        assert!(matches!(result, Ok(_)));
    }
}
