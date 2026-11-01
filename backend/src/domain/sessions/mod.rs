pub mod ports;
pub mod value_objects;

use crate::domain::users::UserId;
use chrono::{DateTime, Duration, Utc};
pub use value_objects::{SessionToken, TokenHash};

#[derive(Clone, Debug)]
pub struct Session {
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub token_hash: TokenHash,
    pub user_id:    UserId,
}

impl Session {
    pub fn start(user_id: UserId, ttl: Duration) -> (Self, SessionToken) {
        let created_at = Utc::now();
        let token      = SessionToken::generate();
        let session    = Self { created_at, expires_at: created_at + ttl, token_hash: token.hash(), user_id };
        (session, token)
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_binds_token_and_expiry() {
        let (session, token) = Session::start(UserId::generate(), Duration::hours(1));
        assert_eq!(session.token_hash, token.hash());
        assert_eq!(session.expires_at - session.created_at, Duration::hours(1));
        assert!(!session.is_expired());
    }

    #[test]
    fn expires_after_ttl() {
        let (session, _) = Session::start(UserId::generate(), Duration::seconds(-1));
        assert!(session.is_expired());
    }
}
