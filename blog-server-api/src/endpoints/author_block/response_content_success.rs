use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "author block state changed")]
pub struct AuthorBlockResponseContentSuccess;
