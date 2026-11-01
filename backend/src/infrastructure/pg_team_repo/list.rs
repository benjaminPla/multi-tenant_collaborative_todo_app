use crate::domain::teams::{ports::TeamRepoError, Team};
use super::row::{map_err, TeamRow};
use sqlx::PgPool;

pub async fn list(pool: &PgPool) -> Result<Vec<Team>, TeamRepoError> {
    let rows = sqlx::query_as::<_, TeamRow>("SELECT created_at, id, name FROM teams ORDER BY name")
        .fetch_all(pool)
        .await
        .map_err(map_err)?;
    rows.into_iter().map(Team::try_from).collect()
}
