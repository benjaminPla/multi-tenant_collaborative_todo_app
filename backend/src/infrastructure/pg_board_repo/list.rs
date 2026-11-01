use crate::{
    domain::{
        boards::{ports::BoardRepoError, Board},
        teams::TeamId,
    },
    infrastructure::tenant,
};
use super::row::{map_err, BoardRow};
use sqlx::PgPool;

pub async fn list(pool: &PgPool, team_id: TeamId) -> Result<Vec<Board>, BoardRepoError> {
    let mut tx = tenant::begin(pool, team_id).await.map_err(map_err)?;
    let rows   = sqlx::query_as::<_, BoardRow>("SELECT created_at, id, name, team_id FROM boards ORDER BY name")
        .fetch_all(&mut *tx)
        .await
        .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    rows.into_iter().map(Board::try_from).collect()
}
