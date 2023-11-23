mod entity_comment_service;
mod entity_post_service;
mod rabbitmq_event_bus_service;
mod rbatis_author_service;
mod rbatis_comment_service;
mod rbatis_post_service;
mod social_service;

pub use entity_comment_service::create_entity_comment_service;
pub use entity_post_service::create_entity_post_service;
pub use rabbitmq_event_bus_service::create_rabbit_event_bus_service;
pub use rbatis_author_service::create_rbatis_author_service;
pub use rbatis_comment_service::create_rbatis_comment_service;
pub use rbatis_post_service::create_rbatis_post_service;
pub use social_service::create_social_service;
