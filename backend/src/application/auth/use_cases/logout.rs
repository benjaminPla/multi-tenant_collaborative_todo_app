use crate::{
    application::auth::errors::AuthAppError,
    domain::sessions::{ports::SessionRepo, SessionToken},
};
use std::sync::Arc;

pub struct LogoutInput {
    pub token: String,
}

pub struct LogoutUseCase {
    session_repo: Arc<dyn SessionRepo>,
}

impl LogoutUseCase {
    pub fn new(session_repo: Arc<dyn SessionRepo>) -> Self {
        Self { session_repo }
    }

    pub async fn execute(&self, input: LogoutInput) -> Result<(), AuthAppError> {
        let Some(token) = SessionToken::parse(&input.token) else {
            return Ok(());
        };
        self.session_repo.delete(&token.hash()).await?;
        Ok(())
    }
}
