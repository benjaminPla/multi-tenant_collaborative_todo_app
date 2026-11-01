use crate::domain::{
    teams::{ports::TeamRepoError, TeamId, TeamRole},
    users::UserId,
};
use super::row::{map_err, parse_role};
use sqlx::PgPool;

pub async fn find_role(pool: &PgPool, id: TeamId, user_id: UserId) -> Result<Option<TeamRole>, TeamRepoError> {
    let role = sqlx::query_scalar::<_, String>(
        "SELECT role::text FROM team_members WHERE team_id = $1 AND user_id = $2",
    )
    .bind(id.as_uuid())
    .bind(user_id.as_uuid())
    .fetch_optional(pool)
    .await
    .map_err(map_err)?;
    role.as_deref().map(parse_role).transpose()
}
