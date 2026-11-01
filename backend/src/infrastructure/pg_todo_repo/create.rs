use crate::{
    domain::todos::{ports::TodoRepoError, Todo},
    infrastructure::tenant,
};
use super::row::{map_err, TodoRow};
use sqlx::PgPool;

pub async fn create(pool: &PgPool, todo: &Todo) -> Result<Todo, TodoRepoError> {
    let mut tx = tenant::begin(pool, todo.team_id).await.map_err(map_err)?;
    let row    = sqlx::query_as::<_, TodoRow>(
        "INSERT INTO todos (assignee_id, board_id, created_at, description, id, status, team_id, title, updated_at, version)
         VALUES ($1, $2, $3, $4, $5, $6::todo_status, $7, $8, $9, $10)
         RETURNING assignee_id, board_id, created_at, description, id, status::text AS status, team_id, title, updated_at, version",
    )
    .bind(todo.assignee_id.map(|id| id.as_uuid()))
    .bind(todo.board_id.as_uuid())
    .bind(todo.created_at)
    .bind(todo.description.as_str())
    .bind(todo.id.as_uuid())
    .bind(todo.status.as_str())
    .bind(todo.team_id.as_uuid())
    .bind(todo.title.as_str())
    .bind(todo.updated_at)
    .bind(todo.version)
    .fetch_one(&mut *tx)
    .await
    .map_err(map_err)?;
    tx.commit().await.map_err(map_err)?;
    row.try_into()
}
