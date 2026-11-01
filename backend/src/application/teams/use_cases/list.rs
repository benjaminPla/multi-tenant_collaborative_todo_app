use crate::{
    application::teams::errors::TeamAppError,
    domain::{
        teams::{ports::TeamRepo, Team},
        users::User,
    },
};
use std::sync::Arc;

pub struct ListTeamsInput {
    pub actor: User,
}

pub struct ListTeamsUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl ListTeamsUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: ListTeamsInput) -> Result<Vec<Team>, TeamAppError> {
        let teams = match input.actor.is_admin {
            true  => self.team_repo.list().await?,
            false => self.team_repo.list_by_user_id(input.actor.id).await?,
        };
        Ok(teams)
    }
}
