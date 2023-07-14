use super::request_content::{LoginRequestContent, LoginRequestContentData};
use super::response_content_failure::LoginResponseContentFailure;
use super::response_content_failure::LoginResponseContentFailure::*;
use super::response_content_success::LoginResponseContentSuccess;
use crate::utils::{auth, password};
use password_hash::Error;

pub async fn http_handler(
    (LoginRequestContent {
        login_data,
        author_service,
    },): (LoginRequestContent,),
) -> Result<LoginResponseContentSuccess, LoginResponseContentFailure> {
    let LoginRequestContentData { slug, password } = login_data.map_err(|e| ParamsDecodeError {
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

    password::verify(&password, &author.base.password_hash).map_err(|e| match e {
        Error::Password => WrongPassword,
        _ => PasswordVerificationError {
            reason: e.to_string(),
        },
    })?;

    let token = auth::token(author).map_err(|e| TokenGeneratingError {
        reason: e.to_string(),
    })?;

    Ok(token.into())
}
