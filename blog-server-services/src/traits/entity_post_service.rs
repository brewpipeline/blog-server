use blog_generic::entities::Post as EPost;
use screw_components::dyn_result::DResult;

use super::post_service::*;

#[async_trait]
pub trait EntityPostService: Send + Sync {
    async fn posts_entities(&self, posts: Vec<Post>) -> DResult<Vec<EPost>>;
}
