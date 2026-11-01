use crate::domain::users::errors::UserError;
use std::fmt;

pub struct Password(String);

impl Password {
    const MAX_LEN: usize = 128;
    const MIN_LEN: usize = 8;

    pub fn parse(raw: String) -> Result<Self, UserError> {
        match raw.chars().count() {
            n if n < Self::MIN_LEN => Err(UserError::PasswordTooShort { min: Self::MIN_LEN }),
            n if n > Self::MAX_LEN => Err(UserError::PasswordTooLong { max: Self::MAX_LEN }),
            _ => Ok(Self(raw)),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Password(***)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforces_length() {
        assert!(matches!(
            Password::parse("short".into()),
            Err(UserError::PasswordTooShort { .. })
        ));
        assert!(matches!(
            Password::parse("a".repeat(Password::MAX_LEN + 1)),
            Err(UserError::PasswordTooLong { .. })
        ));
        assert!(Password::parse("long enough".into()).is_ok());
    }

    #[test]
    fn is_redacted_in_debug() {
        let password = Password::parse("hunter2hunter2".into()).unwrap();
        assert_eq!(format!("{password:?}"), "Password(***)");
    }
}
