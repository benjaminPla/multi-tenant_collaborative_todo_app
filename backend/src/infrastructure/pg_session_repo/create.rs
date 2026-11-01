use crate::domain::sessions::{ports::SessionRepoError, Session};
use super::row::map_err;
use sqlx::PgPool;

pub async fn create(pool: &PgPool, session: &Session) -> Result<(), SessionRepoError> {
    sqlx::query("INSERT INTO sessions (created_at, expires_at, token_hash, user_id) VALUES ($1, $2, $3, $4)")
        .bind(session.created_at)
        .bind(session.expires_at)
        .bind(session.token_hash.as_bytes())
        .bind(session.user_id.as_uuid())
        .execute(pool)
        .await
        .map_err(map_err)?;
    Ok(())
}
