use blog_server_services::traits::author_service::{Author, AuthorService};
use hyper::header::HeaderValue;
use hyper::header::ToStrError;
use hyper::http::request::Parts;
use jsonwebtoken::errors::{Error as JwtError, Result as JwtResult};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt::Display;
use std::future::Future;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    author_id: u64,
    exp: u64,
}

pub fn token(author: Author) -> JwtResult<String> {
    super::jwt::encode(
        &Data {
            author_id: author.id,
            exp: jsonwebtoken::get_current_timestamp() + (60 * 60 * 24 * 31),
        },
        &author.base.password_hash,
    )
}

pub enum Error {
    TokenMissing,
    TokenHeaderCorrupted(ToStrError),
    Token(JwtError),
    DatabaseError(Box<dyn StdError + Send>),
    AuthorNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TokenMissing => write!(f, "token missing"),
            Error::TokenHeaderCorrupted(e) => write!(f, "{}", e.to_string()),
            Error::Token(e) => write!(f, "{}", e.to_string()),
            Error::DatabaseError(e) => write!(f, "{}", e.to_string()),
            Error::AuthorNotFound => write!(f, "author not found"),
        }
    }
}

async fn author_by_token_header_value(
    token_header_value: Option<HeaderValue>,
    author_service: Arc<Box<dyn AuthorService>>,
) -> Result<Author, Error> {
    let token_header_value = token_header_value.ok_or(Error::TokenMissing)?;

    let token = token_header_value
        .to_str()
        .map_err(|e| Error::TokenHeaderCorrupted(e))?;

    let insecure_login_data =
        super::jwt::insecure_decode::<Data>(token).map_err(|e| Error::Token(e))?;

    let author = author_service
        .author_by_id(&insecure_login_data.author_id)
        .await
        .map_err(|e| Error::DatabaseError(e))?
        .ok_or(Error::AuthorNotFound)?;

    super::jwt::decode::<Data>(token, &author.base.password_hash).map_err(|e| Error::Token(e))?;

    Ok(author)
}

pub fn author(
    http_parts: &Parts,
    author_service: Arc<Box<dyn AuthorService>>,
) -> impl Future<Output = Result<Author, Error>> + Send + 'static {
    let token_header_value = http_parts.headers.get("Token").cloned();
    author_by_token_header_value(token_header_value, author_service)
}
