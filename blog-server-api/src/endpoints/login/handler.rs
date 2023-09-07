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

    password::verify(&password, &author.base.password_hash).map_err(|e| {
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
