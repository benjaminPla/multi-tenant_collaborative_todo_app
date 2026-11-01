mod hash;

use crate::domain::{
    hash::{HasherError, HasherService},
    users::{Password, PasswordHash},
};
use async_trait::async_trait;

pub struct Argon2HasherService;

impl Argon2HasherService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HasherService for Argon2HasherService {
    async fn hash(&self, password: &Password) -> Result<PasswordHash, HasherError> {
        hash::hash(password).await
    }
}
