use crate::domain::{
    teams::{ports::TeamRepoError, Team},
    users::UserId,
};
use super::row::{map_err, TeamRow};
use sqlx::PgPool;

pub async fn list_by_user_id(pool: &PgPool, user_id: UserId) -> Result<Vec<Team>, TeamRepoError> {
    let rows = sqlx::query_as::<_, TeamRow>(
        "SELECT teams.created_at, teams.id, teams.name
         FROM teams JOIN team_members ON team_members.team_id = teams.id
         WHERE team_members.user_id = $1
         ORDER BY teams.name",
    )
    .bind(user_id.as_uuid())
    .fetch_all(pool)
    .await
    .map_err(map_err)?;
    rows.into_iter().map(Team::try_from).collect()
}
