use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "post recommended state changed")]
pub struct PostUpdateRecommendedResponseContentSuccess;
