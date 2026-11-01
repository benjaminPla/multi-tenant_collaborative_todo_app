use crate::domain::authz::ports::PipError;

#[derive(Debug, thiserror::Error)]
pub enum AuthzError {
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error(transparent)]
    Pip(#[from] PipError),
}
