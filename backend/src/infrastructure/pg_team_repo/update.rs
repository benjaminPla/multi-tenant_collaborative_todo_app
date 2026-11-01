use crate::domain::teams::{ports::TeamRepoError, Team};
use super::row::{map_err, TeamRow};
use sqlx::PgPool;

pub async fn update(pool: &PgPool, team: &Team) -> Result<Team, TeamRepoError> {
    let row = sqlx::query_as::<_, TeamRow>(
        "UPDATE teams SET name = $2 WHERE id = $1
         RETURNING created_at, id, name",
    )
    .bind(team.id.as_uuid())
    .bind(team.name.as_str())
    .fetch_optional(pool)
    .await
    .map_err(map_err)?;
    row.ok_or(TeamRepoError::NotFound)?.try_into()
}
