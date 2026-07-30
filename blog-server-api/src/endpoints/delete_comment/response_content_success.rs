use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "comment record deleted")]
pub struct DeleteCommentResponseContentSuccess;
