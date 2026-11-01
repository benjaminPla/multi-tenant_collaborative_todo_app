#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum TeamError {
    #[error("team name must be between 1 and {max} characters")]
    InvalidName { max: usize },
    #[error("role must be 'member' or 'owner'")]
    InvalidRole,
}
