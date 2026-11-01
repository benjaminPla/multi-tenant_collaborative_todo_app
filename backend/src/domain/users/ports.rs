use crate::domain::{
    users::{Email, User, UserCredentials, UserId},
    BoxError,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum UserRepoError {
    #[error("email already taken")]
    EmailTaken,
    #[error("user not found")]
    NotFound,
    #[error(transparent)]
    Unexpected(BoxError),
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn create(&self, user: &User)                      -> Result<User, UserRepoError>;
    async fn delete(&self, id: UserId)                       -> Result<(), UserRepoError>;
    async fn find_by_id(&self, id: UserId)                   -> Result<Option<User>, UserRepoError>;
    async fn find_credentials_by_email(&self, email: &Email) -> Result<Option<UserCredentials>, UserRepoError>;
    async fn list(&self)                                     -> Result<Vec<User>, UserRepoError>;
    async fn update(&self, user: &User)                      -> Result<User, UserRepoError>;
}
