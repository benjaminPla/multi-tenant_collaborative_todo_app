use crate::domain::{
    hash::HasherError,
    sessions::ports::SessionRepoError,
    users::ports::UserRepoError,
};

#[derive(Debug, thiserror::Error)]
pub enum AuthnAppError {
    #[error(transparent)]
    Hasher(#[from] HasherError),
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error(transparent)]
    SessionRepo(#[from] SessionRepoError),
    #[error("unauthenticated")]
    Unauthenticated,
    #[error(transparent)]
    UserRepo(#[from] UserRepoError),
}
