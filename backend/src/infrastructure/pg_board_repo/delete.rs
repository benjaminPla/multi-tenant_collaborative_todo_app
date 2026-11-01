use crate::{
    domain::{
        boards::{ports::BoardRepoError, BoardId},
        teams::TeamId,
    },
    infrastructure::tenant,
};
use super::row::map_err;
use sqlx::PgPool;

pub async fn delete(pool: &PgPool, team_id: TeamId, id: BoardId) -> Result<(), BoardRepoError> {
    let mut tx = tenant::begin(pool, team_id).await.map_err(map_err)?;
    let result = sqlx::query("DELETE FROM boards WHERE id = $1")
        .bind(id.as_uuid())
        .execute(&mut *tx)
        .await
        .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    match result.rows_affected() {
        0 => Err(BoardRepoError::NotFound),
        _ => Ok(()),
    }
}
