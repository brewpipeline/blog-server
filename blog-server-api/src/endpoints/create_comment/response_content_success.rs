use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(created, description = "comment record created")]
pub struct CreateCommentContentSuccess;
