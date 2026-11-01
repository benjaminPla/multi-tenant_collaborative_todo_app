use crate::domain::{
    teams::{keeps_an_owner, ports::TeamRepoError, TeamId},
    users::UserId,
};
use super::{lock_member::lock_member, row::map_err};
use sqlx::PgPool;

pub async fn remove_member(pool: &PgPool, id: TeamId, user_id: UserId) -> Result<(), TeamRepoError> {
    let mut tx              = pool.begin().await.map_err(map_err)?;
    let (role, owner_count) = lock_member(&mut tx, id, user_id).await?;
    if !keeps_an_owner(role, None, owner_count) {
        return Err(TeamRepoError::LastOwner);
    }

    sqlx::query("DELETE FROM team_members WHERE team_id = $1 AND user_id = $2")
        .bind(id.as_uuid())
        .bind(user_id.as_uuid())
        .execute(&mut *tx)
        .await
        .map_err(map_err)?;

    tx.commit().await.map_err(map_err)?;
    Ok(())
}
