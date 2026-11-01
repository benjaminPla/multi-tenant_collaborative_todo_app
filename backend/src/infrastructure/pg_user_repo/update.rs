use crate::domain::users::{ports::UserRepoError, User};
use super::row::{map_err, UserRow};
use sqlx::PgPool;

pub async fn update(pool: &PgPool, user: &User) -> Result<User, UserRepoError> {
    let row = sqlx::query_as::<_, UserRow>(
        "UPDATE users SET email = $2, password_hash = $3 WHERE id = $1
         RETURNING created_at, email, id, is_admin, password_hash",
    )
    .bind(user.id.as_uuid())
    .bind(user.email.as_str())
    .bind(user.password_hash.as_str())
    .fetch_optional(pool)
    .await
    .map_err(map_err)?;
    row.ok_or(UserRepoError::NotFound)?.try_into()
}
