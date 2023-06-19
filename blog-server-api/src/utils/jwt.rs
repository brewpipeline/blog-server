use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, errors::Result, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

fn jwt_secret(additional_secret: &String) -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET expected in env vars") + additional_secret.as_str()
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
    Ok(
        jwt_decode::<C>(token, &DecodingKey::from_secret("".as_bytes()), &{
            let mut validation = Validation::default();
            validation.insecure_disable_signature_validation();
            validation
        })?
        .claims,
    )
}
