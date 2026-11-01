use crate::domain::teams::{ports::TeamRepoError, TeamId};
use super::row::map_err;
use sqlx::PgPool;

pub async fn delete(pool: &PgPool, id: TeamId) -> Result<(), TeamRepoError> {
    let result = sqlx::query("DELETE FROM teams WHERE id = $1")
        .bind(id.as_uuid())
        .execute(pool)
        .await
        .map_err(map_err)?;
    match result.rows_affected() {
        0 => Err(TeamRepoError::NotFound),
        _ => Ok(()),
    }
}
