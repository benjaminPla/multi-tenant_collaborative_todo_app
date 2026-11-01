use crate::domain::{
    boards::ports::BoardRepoError,
    todos::{ports::TodoRepoError, TodoError},
};

#[derive(Debug, thiserror::Error)]
pub enum TodoAppError {
    #[error(transparent)]
    Board(#[from] BoardRepoError),
    #[error(transparent)]
    Domain(#[from] TodoError),
    #[error(transparent)]
    Repo(#[from] TodoRepoError),
}
