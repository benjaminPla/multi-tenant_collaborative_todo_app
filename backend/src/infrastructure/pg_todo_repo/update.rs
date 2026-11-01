use crate::{
    domain::todos::{ports::TodoRepoError, Todo},
    infrastructure::tenant,
};
use super::row::{map_err, TodoRow};
use sqlx::PgPool;

pub async fn update(pool: &PgPool, todo: &Todo) -> Result<Todo, TodoRepoError> {
    let mut tx = tenant::begin(pool, todo.team_id).await.map_err(map_err)?;
    let row    = sqlx::query_as::<_, TodoRow>(
        "UPDATE todos
         SET assignee_id = $3, description = $4, status = $5::todo_status, title = $6,
             updated_at = now(), version = version + 1
         WHERE id = $1 AND board_id = $2 AND version = $7
         RETURNING assignee_id, board_id, created_at, description, id, status::text AS status, team_id, title, updated_at, version",
    )
    .bind(todo.id.as_uuid())
    .bind(todo.board_id.as_uuid())
    .bind(todo.assignee_id.map(|id| id.as_uuid()))
    .bind(todo.description.as_str())
    .bind(todo.status.as_str())
    .bind(todo.title.as_str())
    .bind(todo.version)
    .fetch_optional(&mut *tx)
    .await
    .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    row.ok_or(TodoRepoError::VersionConflict)?.try_into()
}
