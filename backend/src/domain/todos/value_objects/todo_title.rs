use crate::domain::todos::errors::TodoError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoTitle(String);

impl TodoTitle {
    const MAX_LEN: usize = 200;

    pub fn parse(raw: &str) -> Result<Self, TodoError> {
        let title = raw.trim();
        match title.chars().count() {
            1..=Self::MAX_LEN => Ok(Self(title.to_owned())),
            _                 => Err(TodoError::InvalidTitle { max: Self::MAX_LEN }),
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
        assert_eq!(TodoTitle::parse("  Deploy to production ").unwrap().as_str(), "Deploy to production");
    }

    #[test]
    fn enforces_length() {
        assert!(TodoTitle::parse("   ").is_err());
        assert!(TodoTitle::parse(&"a".repeat(TodoTitle::MAX_LEN)).is_ok());
        assert!(TodoTitle::parse(&"a".repeat(TodoTitle::MAX_LEN + 1)).is_err());
    }
}
