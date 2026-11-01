use crate::domain::users::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email:    String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email:    Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub created_at: DateTime<Utc>,
    pub email:      String,
    pub id:         Uuid,
    pub is_admin:   bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            created_at: user.created_at,
            email:      user.email.as_str().to_owned(),
            id:         user.id.as_uuid(),
            is_admin:   user.is_admin,
        }
    }
}
