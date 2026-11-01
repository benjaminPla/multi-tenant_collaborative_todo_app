use crate::{
    domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{ports::TodoRepoError, TodoId},
    },
    infrastructure::tenant,
};
use super::row::map_err;
use sqlx::PgPool;

pub async fn delete(pool: &PgPool, team_id: TeamId, board_id: BoardId, id: TodoId) -> Result<(), TodoRepoError> {
    let mut tx = tenant::begin(pool, team_id).await.map_err(map_err)?;
    let result = sqlx::query("DELETE FROM todos WHERE id = $1 AND board_id = $2")
        .bind(id.as_uuid())
        .bind(board_id.as_uuid())
        .execute(&mut *tx)
        .await
        .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    match result.rows_affected() {
        0 => Err(TodoRepoError::NotFound),
        _ => Ok(()),
    }
}
