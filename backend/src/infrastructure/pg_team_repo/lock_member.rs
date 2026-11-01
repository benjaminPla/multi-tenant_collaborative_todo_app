use crate::domain::{
    teams::{ports::TeamRepoError, TeamId, TeamRole},
    users::UserId,
};
use super::row::{map_err, parse_role};
use sqlx::PgConnection;

// team row lock serializes concurrent owner changes, so two owners can't both step down
pub async fn lock_member(conn: &mut PgConnection, id: TeamId, user_id: UserId) -> Result<(TeamRole, i64), TeamRepoError> {
    sqlx::query("SELECT 1 FROM teams WHERE id = $1 FOR NO KEY UPDATE")
        .bind(id.as_uuid())
        .fetch_optional(&mut *conn)
        .await
        .map_err(map_err)?
        .ok_or(TeamRepoError::NotFound)?;

    let (role, owner_count) = sqlx::query_as::<_, (String, i64)>(
        "SELECT role::text, (SELECT count(*) FROM team_members WHERE team_id = $1 AND role = 'owner')
         FROM team_members WHERE team_id = $1 AND user_id = $2",
    )
    .bind(id.as_uuid())
    .bind(user_id.as_uuid())
    .fetch_optional(&mut *conn)
    .await
    .map_err(map_err)?
    .ok_or(TeamRepoError::MemberNotFound)?;

    Ok((parse_role(&role)?, owner_count))
}
