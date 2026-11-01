use crate::{
    application::users::errors::UserAppError,
    domain::{
        hash::HasherService,
        sessions::ports::SessionRepo,
        users::{
            ports::{UserRepo, UserRepoError},
            Email,
            Password,
            User,
            UserId,
        },
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateUserInput {
    pub email:    Option<String>,
    pub id:       Uuid,
    pub password: Option<String>,
}

pub struct UpdateUserUseCase {
    hasher_service: Arc<dyn HasherService>,
    session_repo:   Arc<dyn SessionRepo>,
    user_repo:      Arc<dyn UserRepo>,
}

impl UpdateUserUseCase {
    pub fn new(
        hasher_service: Arc<dyn HasherService>,
        session_repo:   Arc<dyn SessionRepo>,
        user_repo:      Arc<dyn UserRepo>,
    ) -> Self {
        Self { hasher_service, session_repo, user_repo }
    }

    pub async fn execute(&self, input: UpdateUserInput) -> Result<User, UserAppError> {
        let id       = UserId::from(input.id);
        let email    = input.email.as_deref().map(Email::parse).transpose()?;
        let password = input.password.map(Password::parse).transpose()?;
        let mut user = self.user_repo.find_by_id(id).await?.ok_or(UserRepoError::NotFound)?;

        if let Some(email) = email {
            user.email = email;
        }
        let password_changed = password.is_some();
        if let Some(password) = password {
            user.password_hash = self.hasher_service.hash(&password).await?;
        }

        let user = self.user_repo.update(&user).await?;
        if password_changed {
            self.session_repo.delete_by_user_id(user.id).await?;
        }
        Ok(user)
    }
}
