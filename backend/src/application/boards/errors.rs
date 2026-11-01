use crate::domain::boards::{ports::BoardRepoError, BoardError};

#[derive(Debug, thiserror::Error)]
pub enum BoardAppError {
    #[error(transparent)]
    Domain(#[from] BoardError),
    #[error(transparent)]
    Repo(#[from] BoardRepoError),
}
