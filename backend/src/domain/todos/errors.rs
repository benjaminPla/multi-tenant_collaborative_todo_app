#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum TodoError {
    #[error("description must be at most {max} characters")]
    DescriptionTooLong { max: usize },
    #[error("status must be 'todo', 'in-progress' or 'done'")]
    InvalidStatus,
    #[error("title must be between 1 and {max} characters")]
    InvalidTitle { max: usize },
}
