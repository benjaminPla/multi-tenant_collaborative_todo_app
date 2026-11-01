#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum UserError {
    #[error("invalid email")]
    InvalidEmail,
    #[error("password must be at most {max} characters")]
    PasswordTooLong { max: usize },
    #[error("password must be at least {min} characters")]
    PasswordTooShort { min: usize },
}
