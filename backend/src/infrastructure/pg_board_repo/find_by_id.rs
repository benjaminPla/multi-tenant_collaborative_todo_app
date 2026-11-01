use crate::{
    domain::{
        boards::{ports::BoardRepoError, Board, BoardId},
        teams::TeamId,
    },
    infrastructure::tenant,
};
use super::row::{map_err, BoardRow};
use sqlx::PgPool;

pub async fn find_by_id(pool: &PgPool, team_id: TeamId, id: BoardId) -> Result<Option<Board>, BoardRepoError> {
    let mut tx = tenant::begin(pool, team_id).await.map_err(map_err)?;
    let row    = sqlx::query_as::<_, BoardRow>("SELECT created_at, id, name, team_id FROM boards WHERE id = $1")
        .bind(id.as_uuid())
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    row.map(Board::try_from).transpose()
}
