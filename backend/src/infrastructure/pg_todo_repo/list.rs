use crate::{
    domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{ports::TodoRepoError, Todo},
    },
    infrastructure::tenant,
};
use super::row::{map_err, TodoRow};
use sqlx::PgPool;

pub async fn list(pool: &PgPool, team_id: TeamId, board_id: BoardId) -> Result<Vec<Todo>, TodoRepoError> {
    let mut tx = tenant::begin(pool, team_id).await.map_err(map_err)?;
    let rows   = sqlx::query_as::<_, TodoRow>(
        "SELECT assignee_id, board_id, created_at, description, id, status::text AS status, team_id, title, updated_at, version
         FROM todos WHERE board_id = $1 ORDER BY created_at",
    )
    .bind(board_id.as_uuid())
    .fetch_all(&mut *tx)
    .await
    .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    rows.into_iter().map(Todo::try_from).collect()
}
