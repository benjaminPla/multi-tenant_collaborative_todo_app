use crate::{
    application::teams::errors::TeamAppError,
    domain::{
        authz::Scope,
        teams::{ports::TeamRepo, Team},
        users::User,
    },
};
use std::sync::Arc;

pub struct ListTeamsInput {
    pub actor: User,
    pub scope: Scope,
}

pub struct ListTeamsUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl ListTeamsUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: ListTeamsInput) -> Result<Vec<Team>, TeamAppError> {
        let teams = match input.scope {
            Scope::All         => self.team_repo.list().await?,
            Scope::Memberships => self.team_repo.list_by_user_id(input.actor.id).await?,
        };
        Ok(teams)
    }
}
