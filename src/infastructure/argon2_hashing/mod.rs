use anyhow::{Ok, Result};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, Salt, SaltString, rand_core::OsRng},
};

pub fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let byte_password = password.as_bytes();

    let argon2 = Argon2::default();

    let result = argon2
        .hash_password(byte_password, &salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(result.to_string())
}

pub fn verify(password: String, hashed_password: String) -> Result<bool> {
    let parsed_hashed =
        PasswordHash::new(&hashed_password).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let byte_password = password.as_bytes();

    Ok(Argon2::default()
        .verify_password(byte_password, &parsed_hashed)
        .is_ok())
}
