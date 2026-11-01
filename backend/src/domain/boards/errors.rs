#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum BoardError {
    #[error("board name must be between 1 and {max} characters")]
    InvalidName { max: usize },
}
