use crate::domain::{
    hash::HasherError,
    sessions::ports::SessionRepoError,
    users::{
        ports::UserRepoError,
        UserError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum UserAppError {
    #[error(transparent)]
    Domain(#[from] UserError),
    #[error(transparent)]
    Hasher(#[from] HasherError),
    #[error(transparent)]
    Repo(#[from] UserRepoError),
    #[error(transparent)]
    Session(#[from] SessionRepoError),
}
