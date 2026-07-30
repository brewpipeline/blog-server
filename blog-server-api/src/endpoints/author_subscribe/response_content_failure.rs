use blog_server_api_macros::ApiFailure;

use crate::utils::auth_middleware::AuthRejection;

#[derive(ApiFailure)]
pub enum AuthorSubscribeResponseContentFailure {
    #[failure(auth)]
    Auth(AuthRejection),
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(not_found = "author")]
    NotFound,
    #[failure(incorrect_id = "author")]
    IncorrectIdFormat { reason: String },
    #[failure(status = FORBIDDEN, reason = "insufficient rights")]
    Forbidden,
}
