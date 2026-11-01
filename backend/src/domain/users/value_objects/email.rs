use crate::domain::users::errors::UserError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Email(String);

impl Email {
    const MAX_LEN: usize = 254;

    pub fn parse(raw: &str) -> Result<Self, UserError> {
        let email = raw.trim().to_lowercase();
        let valid = email.len() <= Self::MAX_LEN
            && !email.contains(char::is_whitespace)
            && matches!(
                email.split_once('@'),
                Some((local, domain)) if !local.is_empty() && !domain.contains('@') && domain.contains('.')
            );

        valid.then_some(Self(email)).ok_or(UserError::InvalidEmail)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_trimmed_and_lowercased() {
        assert_eq!(
            Email::parse("  Alice@Acme.TEST ").unwrap().as_str(),
            "alice@acme.test"
        );
    }

    #[test]
    fn rejects_invalid() {
        for raw in [
            "",
            "alice",
            "@acme.test",
            "alice@",
            "alice@acme",
            "a@b@c.test",
            "al ice@acme.test",
        ] {
            assert_eq!(Email::parse(raw), Err(UserError::InvalidEmail), "{raw:?}");
        }
    }

    #[test]
    fn rejects_too_long() {
        let raw = format!("{}@acme.test", "a".repeat(Email::MAX_LEN));
        assert_eq!(Email::parse(&raw), Err(UserError::InvalidEmail));
    }
}
