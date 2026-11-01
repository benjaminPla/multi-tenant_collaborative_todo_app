use crate::domain::{
    hash::HasherError,
    users::{Password, PasswordHash},
};
use argon2::{password_hash::phc::PasswordHash as PhcHash, Argon2, PasswordVerifier};

// argon2 is CPU-bound, run off the async runtime
pub async fn verify(password: &Password, hash: &PasswordHash) -> Result<bool, HasherError> {
    let password = password.as_str().to_owned();
    let hash     = hash.as_str().to_owned();
    tokio::task::spawn_blocking(move || {
        let parsed = PhcHash::new(&hash).map_err(|err| HasherError(err.to_string().into()))?;
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
    })
    .await
    .map_err(|err| HasherError(err.into()))?
}
