use blog_server_api_macros::ApiFailure;

use crate::utils::auth_middleware::AuthRejection;

#[derive(ApiFailure)]
pub enum DeleteCommentResponseContentFailure {
    #[failure(auth)]
    Auth(AuthRejection),
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(not_found = "comment")]
    NotFound,
    #[failure(incorrect_id = "comment")]
    IncorrectIdFormat { reason: String },
    #[failure(status = FORBIDDEN, reason = "insufficient rights to delete comment")]
    EditingForbidden,
}
