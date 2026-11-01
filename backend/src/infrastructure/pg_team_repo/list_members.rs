use crate::domain::teams::{ports::TeamRepoError, Member, TeamId};
use super::row::{map_err, MemberRow};
use sqlx::PgPool;

pub async fn list_members(pool: &PgPool, id: TeamId) -> Result<Vec<Member>, TeamRepoError> {
    let rows = sqlx::query_as::<_, MemberRow>(
        "SELECT team_members.created_at, users.email, team_members.role::text AS role, team_members.user_id
         FROM team_members JOIN users ON users.id = team_members.user_id
         WHERE team_members.team_id = $1
         ORDER BY users.email",
    )
    .bind(id.as_uuid())
    .fetch_all(pool)
    .await
    .map_err(map_err)?;
    rows.into_iter().map(Member::try_from).collect()
}
