use std::fmt;

#[derive(Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PasswordHash(***)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_redacted_in_debug() {
        assert_eq!(
            format!("{:?}", PasswordHash::new("$argon2id$...".into())),
            "PasswordHash(***)"
        );
    }
}
