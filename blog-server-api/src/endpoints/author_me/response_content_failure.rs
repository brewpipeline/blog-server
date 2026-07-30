use blog_server_api_macros::ApiFailure;

use crate::utils::auth_middleware::AuthRejection;

#[derive(ApiFailure)]
pub enum AuthorMeResponseContentFailure {
    #[failure(auth)]
    Auth(AuthRejection),
}
