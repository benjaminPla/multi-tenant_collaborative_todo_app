use crate::domain::boards::errors::BoardError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardName(String);

impl BoardName {
    const MAX_LEN: usize = 100;

    pub fn parse(raw: &str) -> Result<Self, BoardError> {
        let name = raw.trim();
        match name.chars().count() {
            1..=Self::MAX_LEN => Ok(Self(name.to_owned())),
            _                 => Err(BoardError::InvalidName { max: Self::MAX_LEN }),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_trimmed() {
        assert_eq!(BoardName::parse("  Website Launch ").unwrap().as_str(), "Website Launch");
    }

    #[test]
    fn enforces_length() {
        assert!(BoardName::parse("   ").is_err());
        assert!(BoardName::parse(&"a".repeat(BoardName::MAX_LEN)).is_ok());
        assert!(BoardName::parse(&"a".repeat(BoardName::MAX_LEN + 1)).is_err());
    }
}
