use crate::{
    application::authn::errors::AuthnAppError,
    domain::{
        sessions::{ports::SessionRepo, SessionToken},
        users::{ports::UserRepo, User},
    },
};
use std::sync::Arc;

pub struct AuthenticateInput {
    pub token: String,
}

pub struct AuthenticateUseCase {
    session_repo: Arc<dyn SessionRepo>,
    user_repo:    Arc<dyn UserRepo>,
}

impl AuthenticateUseCase {
    pub fn new(session_repo: Arc<dyn SessionRepo>, user_repo: Arc<dyn UserRepo>) -> Self {
        Self { session_repo, user_repo }
    }

    pub async fn execute(&self, input: AuthenticateInput) -> Result<User, AuthnAppError> {
        let token      = SessionToken::parse(&input.token).ok_or(AuthnAppError::Unauthenticated)?;
        let token_hash = token.hash();
        let session    = self.session_repo.find_by_token_hash(&token_hash).await?.ok_or(AuthnAppError::Unauthenticated)?;

        if session.is_expired() {
            self.session_repo.delete(&token_hash).await?;
            return Err(AuthnAppError::Unauthenticated);
        }

        let user = self.user_repo.find_by_id(session.user_id).await?.ok_or(AuthnAppError::Unauthenticated)?;
        Ok(user)
    }
}
