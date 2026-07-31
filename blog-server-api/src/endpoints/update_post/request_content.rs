use blog_generic::{entities::CommonPost, events::NewPostPublished};
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::{
    Publish, entity_post_service::EntityPostService, post_service::PostService,
};
use screw_components::dyn_result::DResult;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct UpdatePostRequestContent {
    #[request(path = "id")]
    pub(super) id: Result<u64, std::num::ParseIntError>,
    #[request(data)]
    pub(super) updated_post_data: DResult<CommonPost>,
    #[request(extension)]
    pub(super) post_service: Arc<dyn PostService>,
    #[request(extension)]
    pub(super) entity_post_service: Arc<dyn EntityPostService>,
    #[request(extension)]
    pub(super) new_post_service: Arc<dyn Publish<NewPostPublished>>,
}
