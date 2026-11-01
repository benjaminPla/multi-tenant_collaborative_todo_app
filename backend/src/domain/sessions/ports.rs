use crate::domain::{
    sessions::{Session, TokenHash},
    users::UserId,
    BoxError,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum SessionRepoError {
    #[error(transparent)]
    Unexpected(BoxError),
}

#[async_trait]
pub trait SessionRepo: Send + Sync {
    async fn create(&self, session: &Session)                  -> Result<(), SessionRepoError>;
    async fn delete(&self, token_hash: &TokenHash)             -> Result<(), SessionRepoError>;
    async fn delete_by_user_id(&self, user_id: UserId)         -> Result<(), SessionRepoError>;
    async fn find_by_token_hash(&self, token_hash: &TokenHash) -> Result<Option<Session>, SessionRepoError>;
}
