use crate::domain::{
    teams::{ports::TeamRepoError, TeamError},
    users::UserError,
};

#[derive(Debug, thiserror::Error)]
pub enum TeamAppError {
    #[error(transparent)]
    Domain(#[from] TeamError),
    #[error("forbidden")]
    Forbidden,
    #[error(transparent)]
    Repo(#[from] TeamRepoError),
    #[error(transparent)]
    User(#[from] UserError),
}
