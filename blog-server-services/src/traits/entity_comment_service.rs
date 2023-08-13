use blog_generic::entities::Comment as EComment;
use screw_components::dyn_result::DResult;

use super::comment_service::*;

#[async_trait]
pub trait EntityCommentService: Send + Sync {
    async fn comments_entities(&self, comments: Vec<Comment>) -> DResult<Vec<EComment>>;
}
