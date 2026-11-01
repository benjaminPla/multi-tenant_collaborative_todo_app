use crate::{
    application::teams::errors::TeamAppError,
    domain::teams::{
        ports::{TeamRepo, TeamRepoError},
        Team,
        TeamId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetTeamInput {
    pub id: Uuid,
}

pub struct GetTeamUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl GetTeamUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: GetTeamInput) -> Result<Team, TeamAppError> {
        let id   = TeamId::from(input.id);
        let team = self.team_repo.find_by_id(id).await?.ok_or(TeamRepoError::NotFound)?;
        Ok(team)
    }
}
