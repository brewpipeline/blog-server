use blog_generic::entities::PublishType;
use blog_server_services::traits::author_service::Author;
use blog_server_services::traits::post_service::Post;

pub enum Rejection {
    NotFound,
    Forbidden,
}

pub fn may_publish(author: &Author, publish_type: &PublishType) -> Result<(), String> {
    if !author.is_editor() && publish_type.is_published() {
        Err("publishing not allowed for you".to_owned())
    } else {
        Ok(())
    }
}

pub fn may_edit(author: &Author, post: &Post) -> Result<(), Rejection> {
    if author.is_editor() {
        return Ok(());
    }
    if post.base.publish_type.is_published() {
        return Err(Rejection::Forbidden);
    }
    if author.owns(post.base.author_id) {
        Ok(())
    } else {
        Err(Rejection::NotFound)
    }
}
