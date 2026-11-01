use crate::{
    domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{ports::TodoRepoError, Todo, TodoId},
    },
    infrastructure::tenant,
};
use super::row::{map_err, TodoRow};
use sqlx::PgPool;

pub async fn find_by_id(pool: &PgPool, team_id: TeamId, board_id: BoardId, id: TodoId) -> Result<Option<Todo>, TodoRepoError> {
    let mut tx = tenant::begin(pool, team_id).await.map_err(map_err)?;
    let row    = sqlx::query_as::<_, TodoRow>(
        "SELECT assignee_id, board_id, created_at, description, id, status::text AS status, team_id, title, updated_at, version
         FROM todos WHERE id = $1 AND board_id = $2",
    )
    .bind(id.as_uuid())
    .bind(board_id.as_uuid())
    .fetch_optional(&mut *tx)
    .await
    .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    row.map(Todo::try_from).transpose()
}
