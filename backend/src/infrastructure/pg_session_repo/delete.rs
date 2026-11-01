use crate::domain::sessions::{ports::SessionRepoError, TokenHash};
use super::row::map_err;
use sqlx::PgPool;

pub async fn delete(pool: &PgPool, token_hash: &TokenHash) -> Result<(), SessionRepoError> {
    sqlx::query("DELETE FROM sessions WHERE token_hash = $1")
        .bind(token_hash.as_bytes())
        .execute(pool)
        .await
        .map_err(map_err)?;
    Ok(())
}
