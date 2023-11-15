pub mod author;
pub mod author_block;
pub mod author_me;
pub mod authors;
#[cfg(feature = "ssr")]
mod client_handler;
pub mod comments;
pub mod create_comment;
pub mod create_post;
pub mod delete_comment;
pub mod delete_post;
pub mod login;
pub mod post;
pub mod posts;
#[cfg(feature = "ssr")]
mod sitemap_handler;
pub mod tag;
pub mod update_post;
pub mod yandex_login;
pub mod telegram_login;

#[cfg(feature = "ssr")]
pub use client_handler::*;
#[cfg(feature = "ssr")]
pub use sitemap_handler::*;
