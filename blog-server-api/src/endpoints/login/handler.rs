use super::request_content::{LoginRequestContent, LoginRequestContentData};
use super::response_content_failure::LoginResponseContentFailure;
use super::response_content_failure::LoginResponseContentFailure::*;
use super::response_content_success::LoginResponseContentSuccess;
use crate::extensions::Resolve;
use crate::utils::{login, password};
use blog_server_services::traits::author_service::AuthorService;
use password_hash::Error;
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use screw_components::dyn_result::DResult;
use std::sync::Arc;

async fn handler(
    login_data: DResult<LoginRequestContentData>,
    author_service: Arc<Box<dyn AuthorService>>,
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

    password::verify(&password, &author.password_hash).map_err(|e| match e {
        Error::Password => WrongPassword,
        _ => PasswordVerificationError {
            reason: e.to_string(),
        },
    })?;

    let token = login::token(author).map_err(|e| TokenGeneratingError {
        reason: e.to_string(),
    })?;

    Ok(token.into())
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<LoginRequestContent, Extensions>,
) -> ApiResponse<LoginResponseContentSuccess, LoginResponseContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    ApiResponse::from(handler(request.content.login_data, request.content.author_service).await)
}
