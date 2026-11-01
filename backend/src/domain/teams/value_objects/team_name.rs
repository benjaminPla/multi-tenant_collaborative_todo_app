use crate::domain::teams::errors::TeamError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamName(String);

impl TeamName {
    const MAX_LEN: usize = 100;

    pub fn parse(raw: &str) -> Result<Self, TeamError> {
        let name = raw.trim();
        match name.chars().count() {
            1..=Self::MAX_LEN => Ok(Self(name.to_owned())),
            _                 => Err(TeamError::InvalidName { max: Self::MAX_LEN }),
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
        assert_eq!(TeamName::parse("  Acme Corp ").unwrap().as_str(), "Acme Corp");
    }

    #[test]
    fn enforces_length() {
        assert!(TeamName::parse("   ").is_err());
        assert!(TeamName::parse(&"a".repeat(TeamName::MAX_LEN)).is_ok());
        assert!(TeamName::parse(&"a".repeat(TeamName::MAX_LEN + 1)).is_err());
    }
}
