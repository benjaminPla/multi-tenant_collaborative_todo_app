use crate::domain::users::{ports::UserRepoError, User};
use super::row::{map_err, UserRow};
use sqlx::PgPool;

pub async fn create(pool: &PgPool, user: &User) -> Result<User, UserRepoError> {
    let row = sqlx::query_as::<_, UserRow>(
        "INSERT INTO users (created_at, email, id, is_admin, password_hash)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING created_at, email, id, is_admin, password_hash",
    )
    .bind(user.created_at)
    .bind(user.email.as_str())
    .bind(user.id.as_uuid())
    .bind(user.is_admin)
    .bind(user.password_hash.as_str())
    .fetch_one(pool)
    .await
    .map_err(map_err)?;
    row.try_into()
}
