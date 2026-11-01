use crate::{
    domain::boards::{ports::BoardRepoError, Board},
    infrastructure::tenant,
};
use super::row::{map_err, BoardRow};
use sqlx::PgPool;

pub async fn update(pool: &PgPool, board: &Board) -> Result<Board, BoardRepoError> {
    let mut tx = tenant::begin(pool, board.team_id).await.map_err(map_err)?;
    let row    = sqlx::query_as::<_, BoardRow>(
        "UPDATE boards SET name = $2 WHERE id = $1
         RETURNING created_at, id, name, team_id",
    )
    .bind(board.id.as_uuid())
    .bind(board.name.as_str())
    .fetch_optional(&mut *tx)
    .await
    .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    row.ok_or(BoardRepoError::NotFound)?.try_into()
}
