use blog_generic::entities::LoginYandexQuestion;
use blog_server_services::traits::author_service::BaseMinimalAuthor;
use blog_server_services::traits::social_service::SocialId;
use blog_server_services::utils::time_utils;
use serde::Deserialize;

use super::request_content::LoginYandexRequestContent;
use super::response_content_failure::LoginYandexResponseContentFailure;
use super::response_content_failure::LoginYandexResponseContentFailure::*;
use super::response_content_success::LoginYandexResponseContentSuccess;

use crate::utils::*;

#[derive(Clone, Debug, Deserialize)]
struct YandexLoginPhoneResponse {
    id: u64,
    number: String,
}

#[derive(Clone, Debug, Deserialize)]
struct YandexLoginResponse {
    id: String,
    first_name: Option<String>,
    last_name: Option<String>,
    login: String,
    default_email: Option<String>,
    default_phone: Option<YandexLoginPhoneResponse>,
    #[serde(default)]
    is_avatar_empty: bool,
    #[serde(default)]
    default_avatar_id: String,
}

pub async fn http_handler(
    (LoginYandexRequestContent {
        login_yandex_question,
        social_service,
    },): (LoginYandexRequestContent,),
) -> Result<LoginYandexResponseContentSuccess, LoginYandexResponseContentFailure> {
    let LoginYandexQuestion {
        access_token,
        token_type: _,
        expires_in: _,
    } = login_yandex_question.map_err(|e| ParamsDecodeError {
        reason: e.to_string(),
    })?;

    let yandex_login_response = reqwest::Client::new()
        .get("https://login.yandex.ru/info")
        .header("Authorization", format!("OAuth {access_token}"))
        .send()
        .await
        .map_err(|e| YandexError {
            reason: format!("request: {}", e.to_string()),
        })?
        .json::<YandexLoginResponse>()
        .await
        .map_err(|e| YandexError {
            reason: format!("parse: {}", e.to_string()),
        })?;

    let Some(yandex_id) = yandex_login_response.id.parse::<u64>().ok() else {
        return Err(YandexError {
            reason: "wrong yandex id format".to_string(),
        });
    };

    let yandex_base_minimal_author = BaseMinimalAuthor {
        slug: blog_generic::extend_author_slug(&yandex_login_response.login, &"y".to_string()),
        first_name: yandex_login_response.first_name,
        last_name: yandex_login_response.last_name,
        image_url: if !yandex_login_response.is_avatar_empty {
            Some(format!(
                "https://avatars.yandex.net/get-yapic/{avatar_id}/islands-200?timestamp={timestamp}",
                avatar_id = yandex_login_response.default_avatar_id,
                timestamp = time_utils::now_as_secs()
            ))
        } else {
            None
        },
    };

    let yandex_author = social_service
        .process_auth_by_id(&SocialId::YandexId(yandex_id), &yandex_base_minimal_author)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    let token = auth::token(yandex_author).map_err(|e| TokenGeneratingError {
        reason: e.to_string(),
    })?;

    Ok(token.into())
}
