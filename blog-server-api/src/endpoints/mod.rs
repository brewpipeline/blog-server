pub mod api_catalog_handler;
pub mod author;
pub mod author_block;
pub mod author_me;
pub mod author_override_social_data;
pub mod author_subscribe;
pub mod authors;
#[cfg(feature = "chatgpt")]
pub mod chatgpt;
#[cfg(feature = "ssr")]
mod client_handler;
pub mod comments;
pub mod create_comment;
pub mod create_post;
pub mod delete_comment;
pub mod delete_post;
pub mod login;
#[cfg(feature = "ssr")]
mod markdown_handler;
pub mod openapi_handler;
pub mod post;
pub mod post_recommendation;
pub mod post_update_recommended;
pub mod posts;
#[cfg(feature = "ssr")]
mod robots_handler;
#[cfg(feature = "ssr")]
mod sitemap_handler;
pub mod tag;
#[cfg(feature = "telegram")]
pub mod telegram_login;
pub mod update_minimal_author;
pub mod update_post;
pub mod update_secondary_author;
#[cfg(feature = "yandex")]
pub mod yandex_login;

pub use api_catalog_handler::api_catalog_handler;
pub use openapi_handler::openapi_handler;

#[cfg(feature = "ssr")]
pub use client_handler::*;
#[cfg(feature = "ssr")]
pub use robots_handler::*;
#[cfg(feature = "ssr")]
pub use sitemap_handler::*;
