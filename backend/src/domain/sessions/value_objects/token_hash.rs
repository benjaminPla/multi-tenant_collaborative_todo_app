#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenHash([u8; 32]);

impl TokenHash {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for TokenHash {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}
