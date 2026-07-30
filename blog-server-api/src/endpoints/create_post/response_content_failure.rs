use blog_server_api_macros::ApiFailure;

use crate::utils::auth_middleware::AuthRejection;

#[derive(ApiFailure)]
pub enum CreatePostContentFailure {
    #[failure(auth)]
    Auth(AuthRejection),
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(validation)]
    ValidationError { reason: String },
    #[failure(status = INTERNAL_SERVER_ERROR, reason = "error while creating new post")]
    InsertFailed,
}
