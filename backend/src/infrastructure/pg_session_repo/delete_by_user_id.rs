use crate::domain::{sessions::ports::SessionRepoError, users::UserId};
use super::row::map_err;
use sqlx::PgPool;

pub async fn delete_by_user_id(pool: &PgPool, user_id: UserId) -> Result<(), SessionRepoError> {
    sqlx::query("DELETE FROM sessions WHERE user_id = $1")
        .bind(user_id.as_uuid())
        .execute(pool)
        .await
        .map_err(map_err)?;
    Ok(())
}
