use blog_server_api_macros::ApiFailure;

use crate::utils::auth_middleware::AuthRejection;

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
