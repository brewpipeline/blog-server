use super::request_content::AuthorRequestContent;
use super::response_content_failure::AuthorResponseContentFailure;
use super::response_content_failure::AuthorResponseContentFailure::*;
use super::response_content_success::AuthorResponseContentSuccess;
use crate::extensions::Resolve;
use blog_server_services::traits::user_service::UserService;
use screw_api::request::ApiRequest;
use screw_api::response::{ApiResponse, ApiResponseContent, ApiResponseContent::*};
use std::sync::Arc;

async fn handler(
    authorname: Option<String>,
    user_service: Arc<Box<dyn UserService>>,
) -> ApiResponseContent<AuthorResponseContentSuccess, AuthorResponseContentFailure> {
    let Some(username) = authorname else {
        return Failure(NameMissing)
    };
    if username.is_empty() {
        return Failure(NameMissing);
    }
    match user_service.get_user(&username).await {
        Ok(user) => {
            if let Some(user) = user {
                Success(user.into())
            } else {
                Failure(NotFound)
            }
        }
        Err(err) => Failure(DatabaseError {
            reason: err.to_string(),
        }),
    }
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<AuthorRequestContent, Extensions>,
) -> ApiResponse<AuthorResponseContentSuccess, AuthorResponseContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn UserService>>>,
{
    ApiResponse {
        content: handler(request.content.authorname, request.content.user_service).await,
    }
}
