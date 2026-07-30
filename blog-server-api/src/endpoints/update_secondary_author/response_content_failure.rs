use blog_server_api_macros::ApiFailure;

use crate::utils::auth_middleware::AuthRejection;

#[derive(ApiFailure)]
pub enum UpdateSecondaryAuthorContentFailure {
    #[failure(auth)]
    Auth(AuthRejection),
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(validation)]
    ValidationError { reason: String },
}
