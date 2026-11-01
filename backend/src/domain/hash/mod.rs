use crate::domain::{
    users::{Password, PasswordHash},
    BoxError,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
#[error("hasher failure")]
pub struct HasherError(#[source] pub BoxError);

#[async_trait]
pub trait HasherService: Send + Sync {
    async fn hash(&self, password: &Password)                        -> Result<PasswordHash, HasherError>;
    async fn verify(&self, password: &Password, hash: &PasswordHash) -> Result<bool, HasherError>;
    async fn verify_dummy(&self, password: &Password)                -> Result<(), HasherError>;
}
