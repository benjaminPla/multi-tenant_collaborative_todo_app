use crate::{
    application::users::errors::UserAppError,
    domain::{
        hash::HasherService,
        users::{
            ports::UserRepo,
            Email,
            Password,
            User,
        },
    },
};
use std::sync::Arc;

pub struct CreateUserInput {
    pub email:    String,
    pub password: String,
}

pub struct CreateUserUseCase {
    hasher_service: Arc<dyn HasherService>,
    user_repo:      Arc<dyn UserRepo>,
}

impl CreateUserUseCase {
    pub fn new(hasher_service: Arc<dyn HasherService>, user_repo: Arc<dyn UserRepo>) -> Self {
        Self { hasher_service, user_repo }
    }

    pub async fn execute(&self, input: CreateUserInput) -> Result<User, UserAppError> {
        let email         = Email::parse(&input.email)?;
        let password      = Password::parse(input.password)?;
        let password_hash = self.hasher_service.hash(&password).await?;
        let user          = User::register(email, password_hash);
        let user          = self.user_repo.create(&user).await?;
        Ok(user)
    }
}
