use crate::domain::teams::{ports::TeamRepoError, TeamError};

#[derive(Debug, thiserror::Error)]
pub enum TeamAppError {
    #[error(transparent)]
    Domain(#[from] TeamError),
    #[error("forbidden")]
    Forbidden,
    #[error(transparent)]
    Repo(#[from] TeamRepoError),
}
