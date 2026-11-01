use crate::domain::users::{ports::UserRepoError, User};
use super::row::{map_err, UserRow};
use sqlx::PgPool;

pub async fn list(pool: &PgPool) -> Result<Vec<User>, UserRepoError> {
    let rows = sqlx::query_as::<_, UserRow>(
        "SELECT created_at, email, id, is_admin, password_hash FROM users ORDER BY created_at",
    )
    .fetch_all(pool)
    .await
    .map_err(map_err)?;
    rows.into_iter().map(User::try_from).collect()
}
