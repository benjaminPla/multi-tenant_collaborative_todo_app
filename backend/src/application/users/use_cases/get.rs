use crate::{
    application::users::errors::UserAppError,
    domain::users::{
        ports::{UserRepo, UserRepoError},
        User,
        UserId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserInput {
    pub id: Uuid,
}

pub struct GetUserUseCase {
    user_repo: Arc<dyn UserRepo>,
}

impl GetUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: GetUserInput) -> Result<User, UserAppError> {
        let id   = UserId::from(input.id);
        let user = self.user_repo.find_by_id(id).await?.ok_or(UserRepoError::NotFound)?;
        Ok(user)
    }
}
