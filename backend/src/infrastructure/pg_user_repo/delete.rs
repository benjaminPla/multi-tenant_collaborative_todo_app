use crate::domain::users::{ports::UserRepoError, UserId};
use super::row::map_err;
use sqlx::PgPool;

pub async fn delete(pool: &PgPool, id: UserId) -> Result<(), UserRepoError> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id.as_uuid())
        .execute(pool)
        .await
        .map_err(map_err)?;
    match result.rows_affected() {
        0 => Err(UserRepoError::NotFound),
        _ => Ok(()),
    }
}
