use crate::{
    domain::boards::{ports::BoardRepoError, Board},
    infrastructure::tenant,
};
use super::row::{map_err, BoardRow};
use sqlx::PgPool;

pub async fn create(pool: &PgPool, board: &Board) -> Result<Board, BoardRepoError> {
    let mut tx = tenant::begin(pool, board.team_id).await.map_err(map_err)?;
    let row    = sqlx::query_as::<_, BoardRow>(
        "INSERT INTO boards (created_at, id, name, team_id) VALUES ($1, $2, $3, $4)
         RETURNING created_at, id, name, team_id",
    )
    .bind(board.created_at)
    .bind(board.id.as_uuid())
    .bind(board.name.as_str())
    .bind(board.team_id.as_uuid())
    .fetch_one(&mut *tx)
    .await
    .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    row.try_into()
}
