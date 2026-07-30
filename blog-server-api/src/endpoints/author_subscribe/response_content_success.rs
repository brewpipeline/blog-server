use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "author notification subscription state changed")]
pub struct AuthorSubscribeRequestContentSuccess;
