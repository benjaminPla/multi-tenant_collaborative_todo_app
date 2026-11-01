use crate::{
    application::users::errors::UserAppError,
    domain::users::{
        ports::UserRepo,
        User,
    },
};
use std::sync::Arc;

pub struct ListUsersUseCase {
    user_repo: Arc<dyn UserRepo>,
}

impl ListUsersUseCase {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self) -> Result<Vec<User>, UserAppError> {
        let users = self.user_repo.list().await?;
        Ok(users)
    }
}
