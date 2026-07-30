use blog_server_api_macros::ApiFailure;

use crate::utils::auth_middleware::AuthRejection;

#[derive(ApiFailure)]
pub enum PostsResponseContentFailure {
    #[failure(auth)]
    Auth(AuthRejection),
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(status = FORBIDDEN, reason = "insufficient rights")]
    Forbidden,
}
