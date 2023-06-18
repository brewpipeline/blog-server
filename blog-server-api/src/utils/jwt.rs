use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, errors::Result, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};

const SECRET: &'static str = "";

pub fn encode(claims: &impl Serialize, additional_secret: &String) -> Result<String> {
    jwt_encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret((additional_secret.to_owned() + SECRET).as_bytes()),
    )
}

pub fn decode<C: for<'de> Deserialize<'de>>(
    token: &String,
    additional_secret: &String,
) -> Result<C> {
    Ok(jwt_decode::<C>(
        token,
        &DecodingKey::from_secret((additional_secret.to_owned() + SECRET).as_bytes()),
        &Validation::default(),
    )?
    .claims)
}
