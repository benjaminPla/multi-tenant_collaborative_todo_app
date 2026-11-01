use crate::domain::users::{ports::UserRepoError, User, UserId};
use super::row::{map_err, UserRow};
use sqlx::PgPool;

pub async fn find_by_id(pool: &PgPool, id: UserId) -> Result<Option<User>, UserRepoError> {
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT created_at, email, id, is_admin, password_hash FROM users WHERE id = $1",
    )
    .bind(id.as_uuid())
    .fetch_optional(pool)
    .await
    .map_err(map_err)?;
    row.map(User::try_from).transpose()
}
