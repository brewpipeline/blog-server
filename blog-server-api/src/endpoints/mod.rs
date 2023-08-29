pub mod author;
pub mod author_me;
pub mod authors;
#[cfg(feature = "ssr")]
mod client_handler;
pub mod comments;
pub mod create_post;
pub mod login;
pub mod post;
pub mod posts;
pub mod tag;
pub mod update_post;

#[cfg(feature = "ssr")]
pub use client_handler::*;
