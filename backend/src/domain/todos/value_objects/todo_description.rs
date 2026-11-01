use crate::domain::todos::errors::TodoError;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TodoDescription(String);

impl TodoDescription {
    const MAX_LEN: usize = 5000;

    pub fn parse(raw: String) -> Result<Self, TodoError> {
        match raw.chars().count() {
            0..=Self::MAX_LEN => Ok(Self(raw)),
            _                 => Err(TodoError::DescriptionTooLong { max: Self::MAX_LEN }),
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
    fn enforces_length() {
        assert!(TodoDescription::parse(String::new()).is_ok());
        assert!(TodoDescription::parse("a".repeat(TodoDescription::MAX_LEN)).is_ok());
        assert!(TodoDescription::parse("a".repeat(TodoDescription::MAX_LEN + 1)).is_err());
    }
}
