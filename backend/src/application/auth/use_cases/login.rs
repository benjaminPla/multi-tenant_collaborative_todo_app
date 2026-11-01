use crate::{
    application::auth::errors::AuthAppError,
    domain::{
        hash::HasherService,
        sessions::{
            ports::SessionRepo,
            Session,
            SessionToken,
        },
        users::{
            ports::UserRepo,
            Email,
            Password,
        },
    },
};
use chrono::Duration;
use std::sync::Arc;

pub struct LoginInput {
    pub email:    String,
    pub password: String,
}

pub struct LoginUseCase {
    hasher_service: Arc<dyn HasherService>,
    session_repo:   Arc<dyn SessionRepo>,
    session_ttl:    Duration,
    user_repo:      Arc<dyn UserRepo>,
}

impl LoginUseCase {
    pub fn new(
        hasher_service: Arc<dyn HasherService>,
        session_repo:   Arc<dyn SessionRepo>,
        session_ttl:    Duration,
        user_repo:      Arc<dyn UserRepo>,
    ) -> Self {
        Self { hasher_service, session_repo, session_ttl, user_repo }
    }

    pub async fn execute(&self, input: LoginInput) -> Result<SessionToken, AuthAppError> {
        let email    = Email::parse(&input.email).map_err(|_| AuthAppError::InvalidCredentials)?;
        let password = Password::parse(input.password).map_err(|_| AuthAppError::InvalidCredentials)?;

        let Some(credentials) = self.user_repo.find_credentials_by_email(&email).await? else {
            self.hasher_service.verify_dummy(&password).await?;
            return Err(AuthAppError::InvalidCredentials);
        };
        if !self.hasher_service.verify(&password, &credentials.password_hash).await? {
            return Err(AuthAppError::InvalidCredentials);
        }

        let (session, token) = Session::start(credentials.id, self.session_ttl);
        self.session_repo.create(&session).await?;
        Ok(token)
    }
}
