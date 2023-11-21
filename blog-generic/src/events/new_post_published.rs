use serde::Serialize;

#[derive(Serialize)]
pub struct NewPostPublished {
    pub blog_user_id: u64,
    pub post_sub_url: String,
}
