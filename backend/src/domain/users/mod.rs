pub mod errors;
pub mod ports;
pub mod value_objects;

use chrono::{DateTime, Utc};
pub use errors::UserError;
pub use value_objects::{Email, Password, PasswordHash, UserId};

#[derive(Clone, Debug)]
pub struct User {
    pub created_at: DateTime<Utc>,
    pub email: Email,
    pub id: UserId,
    pub is_admin: bool,
    pub password_hash: PasswordHash,
}

#[derive(Clone, Debug)]
pub struct UserCredentials {
    pub id: UserId,
    pub password_hash: PasswordHash,
}

impl User {
    pub fn register(email: Email, password_hash: PasswordHash) -> Self {
        Self {
            created_at: Utc::now(),
            email,
            id: UserId::generate(),
            is_admin: false,
            password_hash,
        }
    }
}
