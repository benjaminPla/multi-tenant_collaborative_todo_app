use crate::domain::{
    teams::{ports::TeamRepoError, Team, TeamRole},
    users::UserId,
};
use super::row::{map_err, TeamRow};
use sqlx::PgPool;

pub async fn create(pool: &PgPool, team: &Team, owner_id: UserId) -> Result<Team, TeamRepoError> {
    let mut tx = pool.begin().await.map_err(map_err)?;

    let row = sqlx::query_as::<_, TeamRow>(
        "INSERT INTO teams (created_at, id, name) VALUES ($1, $2, $3)
         RETURNING created_at, id, name",
    )
    .bind(team.created_at)
    .bind(team.id.as_uuid())
    .bind(team.name.as_str())
    .fetch_one(&mut *tx)
    .await
    .map_err(map_err)?;

    sqlx::query("INSERT INTO team_members (role, team_id, user_id) VALUES ($1::team_role, $2, $3)")
        .bind(TeamRole::Owner.as_str())
        .bind(team.id.as_uuid())
        .bind(owner_id.as_uuid())
        .execute(&mut *tx)
        .await
        .map_err(map_err)?;

    tx.commit().await.map_err(map_err)?;
    row.try_into()
}
