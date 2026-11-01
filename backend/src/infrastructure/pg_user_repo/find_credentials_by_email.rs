use crate::domain::users::{ports::UserRepoError, Email, UserCredentials};
use super::row::{map_err, CredentialsRow};
use sqlx::PgPool;

pub async fn find_credentials_by_email(pool: &PgPool, email: &Email) -> Result<Option<UserCredentials>, UserRepoError> {
    let row = sqlx::query_as::<_, CredentialsRow>("SELECT id, password_hash FROM users WHERE email = $1")
        .bind(email.as_str())
        .fetch_optional(pool)
        .await
        .map_err(map_err)?;
    Ok(row.map(UserCredentials::from))
}
