use crate::domain::{
    hash::HasherError,
    users::{Password, PasswordHash},
};
use argon2::{Argon2, PasswordHasher};

pub async fn hash(password: &Password) -> Result<PasswordHash, HasherError> {
    let password = password.as_str().to_owned();
    tokio::task::spawn_blocking(move || hash_blocking(&password))
        .await
        .map_err(|err| HasherError(err.into()))?
}

pub fn hash_blocking(password: &str) -> Result<PasswordHash, HasherError> {
    let hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|err| HasherError(err.to_string().into()))?;
    Ok(PasswordHash::new(hash.to_string()))
}
