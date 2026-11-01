use crate::domain::{
    authz::ports::PipError,
    teams::{TeamId, TeamRole},
    users::UserId,
};
use sqlx::PgPool;

pub async fn team_role(pool: &PgPool, team_id: TeamId, user_id: UserId) -> Result<Option<TeamRole>, PipError> {
    let role = sqlx::query_scalar::<_, String>(
        "SELECT role::text FROM team_members WHERE team_id = $1 AND user_id = $2",
    )
    .bind(team_id.as_uuid())
    .bind(user_id.as_uuid())
    .fetch_optional(pool)
    .await
    .map_err(|err| PipError(err.into()))?;
    role.as_deref()
        .map(TeamRole::parse)
        .transpose()
        .map_err(|err| PipError(err.into()))
}
