use crate::{
    application::users::errors::UserAppError,
    domain::users::{
        ports::UserRepo,
        UserId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteUserInput {
    pub id: Uuid,
}

pub struct DeleteUserUseCase {
    user_repo: Arc<dyn UserRepo>,
}

impl DeleteUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: DeleteUserInput) -> Result<(), UserAppError> {
        let id = UserId::from(input.id);
        self.user_repo.delete(id).await?;
        Ok(())
    }
}
