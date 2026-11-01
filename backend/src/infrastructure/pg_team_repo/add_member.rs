use crate::domain::{
    teams::{ports::TeamRepoError, Member, TeamId, TeamRole},
    users::Email,
};
use super::row::{map_err, MemberRow};
use sqlx::PgPool;

pub async fn add_member(pool: &PgPool, id: TeamId, email: &Email, role: TeamRole) -> Result<Member, TeamRepoError> {
    let row = sqlx::query_as::<_, MemberRow>(
        "WITH member AS (
             INSERT INTO team_members (role, team_id, user_id)
             SELECT $3::team_role, $1, id FROM users WHERE email = $2
             RETURNING created_at, role, user_id
         )
         SELECT member.created_at, users.email, member.role::text AS role, member.user_id
         FROM member JOIN users ON users.id = member.user_id",
    )
    .bind(id.as_uuid())
    .bind(email.as_str())
    .bind(role.as_str())
    .fetch_optional(pool)
    .await
    .map_err(map_err)?;
    row.ok_or(TeamRepoError::UserNotFound)?.try_into()
}
