use crate::domain::sessions::{ports::SessionRepoError, Session, TokenHash};
use super::row::{map_err, SessionRow};
use sqlx::PgPool;

pub async fn find_by_token_hash(pool: &PgPool, token_hash: &TokenHash) -> Result<Option<Session>, SessionRepoError> {
    let row = sqlx::query_as::<_, SessionRow>(
        "SELECT created_at, expires_at, token_hash, user_id FROM sessions WHERE token_hash = $1",
    )
    .bind(token_hash.as_bytes())
    .fetch_optional(pool)
    .await
    .map_err(map_err)?;
    row.map(Session::try_from).transpose()
}
