use blog_server_api_macros::ApiFailure;
use blog_server_services::traits::social_service::SubscribeRejection;

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
    #[failure(status = CONFLICT, reason = "author has no channel to be notified through")]
    NoNotificationChannel,
}

impl From<SubscribeRejection> for AuthorSubscribeResponseContentFailure {
    fn from(rejection: SubscribeRejection) -> Self {
        match rejection {
            SubscribeRejection::NoNotificationChannel => Self::NoNotificationChannel,
            SubscribeRejection::Other(error) => Self::DatabaseError {
                reason: error.to_string(),
            },
        }
    }
}
