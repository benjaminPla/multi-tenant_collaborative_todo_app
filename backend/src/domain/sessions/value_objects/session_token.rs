use crate::domain::sessions::TokenHash;
use sha2::{Digest, Sha256};
use std::fmt::{self, Write};

pub struct SessionToken(String);

impl SessionToken {
    const BYTES: usize = 32;

    pub fn generate() -> Self {
        let bytes: [u8; Self::BYTES] = rand::random();
        let mut hex                  = String::with_capacity(Self::BYTES * 2);
        for byte in bytes {
            let _ = write!(hex, "{byte:02x}");
        }
        Self(hex)
    }

    pub fn parse(raw: &str) -> Option<Self> {
        let valid = raw.len() == Self::BYTES * 2 && raw.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'));
        valid.then(|| Self(raw.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn hash(&self) -> TokenHash {
        TokenHash::from(<[u8; 32]>::from(Sha256::digest(self.0.as_bytes())))
    }
}

impl fmt::Debug for SessionToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SessionToken(***)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_unique_parsable_tokens() {
        let token = SessionToken::generate();
        assert!(SessionToken::parse(token.as_str()).is_some());
        assert_ne!(token.as_str(), SessionToken::generate().as_str());
    }

    #[test]
    fn hash_is_deterministic() {
        let token = SessionToken::generate();
        assert_eq!(token.hash(), SessionToken::parse(token.as_str()).unwrap().hash());
    }

    #[test]
    fn is_redacted_in_debug() {
        assert_eq!(format!("{:?}", SessionToken::generate()), "SessionToken(***)");
    }

    #[test]
    fn rejects_invalid() {
        for raw in ["", "abc", &"A".repeat(64), &"g".repeat(64), &"a".repeat(65)] {
            assert!(SessionToken::parse(raw).is_none(), "{raw:?}");
        }
    }
}
