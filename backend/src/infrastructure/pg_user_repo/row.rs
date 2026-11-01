use crate::domain::users::{
    ports::UserRepoError,
    Email,
    PasswordHash,
    User,
    UserId,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct UserRow {
    created_at:    DateTime<Utc>,
    email:         String,
    id:            Uuid,
    is_admin:      bool,
    password_hash: String,
}

impl TryFrom<UserRow> for User {
    type Error = UserRepoError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(Self {
            created_at:    row.created_at,
            email:         Email::parse(&row.email).map_err(|err| UserRepoError::Unexpected(err.into()))?,
            id:            UserId::from(row.id),
            is_admin:      row.is_admin,
            password_hash: PasswordHash::new(row.password_hash),
        })
    }
}

pub fn map_err(err: sqlx::Error) -> UserRepoError {
    match err.as_database_error() {
        Some(db_err) if db_err.is_unique_violation() => UserRepoError::EmailTaken,
        _                                            => UserRepoError::Unexpected(err.into()),
    }
}
