use crate::domain::teams::{ports::TeamRepoError, Team, TeamId};
use super::row::{map_err, TeamRow};
use sqlx::PgPool;

pub async fn find_by_id(pool: &PgPool, id: TeamId) -> Result<Option<Team>, TeamRepoError> {
    let row = sqlx::query_as::<_, TeamRow>("SELECT created_at, id, name FROM teams WHERE id = $1")
        .bind(id.as_uuid())
        .fetch_optional(pool)
        .await
        .map_err(map_err)?;
    row.map(Team::try_from).transpose()
}
