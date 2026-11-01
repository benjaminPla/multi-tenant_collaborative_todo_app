use crate::domain::{
    teams::{keeps_an_owner, ports::TeamRepoError, Member, TeamId, TeamRole},
    users::UserId,
};
use super::{
    lock_member::lock_member,
    row::{map_err, MemberRow},
};
use sqlx::PgPool;

pub async fn update_member_role(pool: &PgPool, id: TeamId, user_id: UserId, role: TeamRole) -> Result<Member, TeamRepoError> {
    let mut tx                 = pool.begin().await.map_err(map_err)?;
    let (current, owner_count) = lock_member(&mut tx, id, user_id).await?;
    if !keeps_an_owner(current, Some(role), owner_count) {
        return Err(TeamRepoError::LastOwner);
    }

    let row = sqlx::query_as::<_, MemberRow>(
        "UPDATE team_members SET role = $3::team_role
         FROM users
         WHERE team_members.team_id = $1 AND team_members.user_id = $2 AND users.id = team_members.user_id
         RETURNING team_members.created_at, users.email, team_members.role::text AS role, team_members.user_id",
    )
    .bind(id.as_uuid())
    .bind(user_id.as_uuid())
    .bind(role.as_str())
    .fetch_one(&mut *tx)
    .await
    .map_err(map_err)?;

    tx.commit().await.map_err(map_err)?;
    row.try_into()
}
