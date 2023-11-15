use blog_generic::entities::LoginTelegramQuestion;
use blog_server_services::traits::author_service::BaseAuthor;
use blog_server_services::utils::time_utils;
use hmac::Mac;
use sha2::{Digest, Sha256};

use super::request_content::LoginTelegramRequestContent;
use super::response_content_failure::LoginTelegramResponseContentFailure;
use super::response_content_failure::LoginTelegramResponseContentFailure::*;
use super::response_content_success::LoginTelegramResponseContentSuccess;

use crate::utils::*;

fn sha256(input: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

fn hmac_sha256(key: &[u8], data: &str) -> String {
    let mut mac =
        hmac::Hmac::<sha2::Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

pub async fn http_handler(
    (LoginTelegramRequestContent {
        login_telegram_question,
        author_service,
    },): (LoginTelegramRequestContent,),
) -> Result<LoginTelegramResponseContentSuccess, LoginTelegramResponseContentFailure> {
    let LoginTelegramQuestion {
        id,
        first_name,
        last_name,
        username,
        photo_url,
        auth_date,
        hash,
    } = login_telegram_question.map_err(|e| ParamsDecodeError {
        reason: e.to_string(),
    })?;

    let secret_key = sha256(crate::TELEGRAM_BOT_TOKEN);
    let check_string = {
        let mut parts = vec![
            format!("id={}", id),
            format!("first_name={}", first_name),
            format!("last_name={}", last_name),
            format!("auth_date={}", auth_date),
        ];

        if let Some(username) = &username {
            parts.push(format!("username={}", username));
        }

        if let Some(photo_url) = &photo_url {
            parts.push(format!("photo_url={}", photo_url));
        }

        parts.sort();
        parts.join("\n")
    };

    let computed_hash = hmac_sha256(&secret_key, &check_string);

    if computed_hash != hash {
        return Err(TelegramError {
            reason: "incorrect signature".to_string(),
        });
    }

    let telegram_base_author = BaseAuthor {
        slug: username.unwrap_or(id.to_string()),
        first_name: Some(first_name),
        middle_name: None,
        last_name: Some(last_name),
        mobile: None,
        email: None,
        password_hash: None,
        registered_at: time_utils::now_as_secs(),
        status: None,
        image_url: photo_url,
        editor: 0,
        blocked: 0,
        yandex_id: None,
        telegram_id: Some(id),
    };

    let telegram_author_id = author_service
        .create_or_update_telegram_author(&telegram_base_author)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    let telegram_author = author_service
        .author_by_id(&telegram_author_id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(DatabaseError {
            reason: "insert error".to_string(),
        })?;

    let token = auth::token(telegram_author).map_err(|e| TokenGeneratingError {
        reason: e.to_string(),
    })?;

    Ok(token.into())
}
