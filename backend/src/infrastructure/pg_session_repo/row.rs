use crate::domain::{
    sessions::{ports::SessionRepoError, Session, TokenHash},
    users::UserId,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct SessionRow {
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    token_hash: Vec<u8>,
    user_id:    Uuid,
}

impl TryFrom<SessionRow> for Session {
    type Error = SessionRepoError;

    fn try_from(row: SessionRow) -> Result<Self, Self::Error> {
        let token_hash = <[u8; 32]>::try_from(row.token_hash)
            .map_err(|_| SessionRepoError::Unexpected("token_hash is not 32 bytes".into()))?;
        Ok(Self {
            created_at: row.created_at,
            expires_at: row.expires_at,
            token_hash: TokenHash::from(token_hash),
            user_id:    UserId::from(row.user_id),
        })
    }
}

pub fn map_err(err: sqlx::Error) -> SessionRepoError {
    SessionRepoError::Unexpected(err.into())
}
