use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Result, SaltString,
    },
    Argon2,
};

pub fn hash(password: &String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

pub fn verify(password: &String, password_hash: &String) -> Result<()> {
    let password_hash = PasswordHash::new(password_hash)?;

    Argon2::default().verify_password(password.as_bytes(), &password_hash)?;

    Ok(())
}
