use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation,
    dangerous::insecure_decode as jwt_insecure_decode, decode as jwt_decode, encode as jwt_encode,
    errors::Result,
};
use serde::{Deserialize, Serialize};

fn jwt_secret(additional_secret: &String) -> String {
    crate::JWT_SECRET.to_string() + additional_secret
}

pub fn encode(claims: &impl Serialize, additional_secret: &String) -> Result<String> {
    jwt_encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(jwt_secret(additional_secret).as_bytes()),
    )
}

pub fn decode<C: for<'de> Deserialize<'de>>(token: &str, additional_secret: &String) -> Result<C> {
    Ok(jwt_decode::<C>(
        token,
        &DecodingKey::from_secret(jwt_secret(additional_secret).as_bytes()),
        &Validation::default(),
    )?
    .claims)
}

pub fn insecure_decode<C: for<'de> Deserialize<'de>>(token: &str) -> Result<C> {
    Ok(jwt_insecure_decode::<C>(token)?.claims)
}
