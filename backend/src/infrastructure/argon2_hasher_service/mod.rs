mod hash;
mod verify;

use crate::domain::{
    hash::{HasherError, HasherService},
    users::{Password, PasswordHash},
};
use async_trait::async_trait;

pub struct Argon2HasherService {
    dummy_hash: PasswordHash,
}

impl Argon2HasherService {
    pub fn new() -> Self {
        let dummy_hash = hash::hash_blocking("dummy-password").expect("Failed to hash dummy password");
        Self { dummy_hash }
    }
}

#[async_trait]
impl HasherService for Argon2HasherService {
    async fn hash(&self, password: &Password) -> Result<PasswordHash, HasherError> {
        hash::hash(password).await
    }

    async fn verify(&self, password: &Password, hash: &PasswordHash) -> Result<bool, HasherError> {
        verify::verify(password, hash).await
    }

    async fn verify_dummy(&self, password: &Password) -> Result<(), HasherError> {
        verify::verify(password, &self.dummy_hash).await.map(|_| ())
    }
}
