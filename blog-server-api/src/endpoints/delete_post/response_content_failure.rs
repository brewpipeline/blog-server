use blog_server_api_macros::ApiFailure;

use crate::utils::auth_middleware::AuthRejection;
use crate::utils::post_access;

#[derive(ApiFailure)]
pub enum DeletePostResponseContentFailure {
    #[failure(auth)]
    Auth(AuthRejection),
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(not_found = "post")]
    NotFound,
    #[failure(incorrect_id = "post")]
    IncorrectIdFormat { reason: String },
    #[failure(status = FORBIDDEN, reason = "insufficient rights to delete post")]
    EditingForbidden,
}

impl From<post_access::Rejection> for DeletePostResponseContentFailure {
    fn from(value: post_access::Rejection) -> Self {
        match value {
            post_access::Rejection::NotFound => Self::NotFound,
            post_access::Rejection::Forbidden => Self::EditingForbidden,
        }
    }
}
