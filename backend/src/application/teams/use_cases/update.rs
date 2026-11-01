use crate::{
    application::teams::errors::TeamAppError,
    domain::teams::{
        ports::{TeamRepo, TeamRepoError},
        Team,
        TeamId,
        TeamName,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateTeamInput {
    pub id:   Uuid,
    pub name: String,
}

pub struct UpdateTeamUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl UpdateTeamUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: UpdateTeamInput) -> Result<Team, TeamAppError> {
        let id   = TeamId::from(input.id);
        let name = TeamName::parse(&input.name)?;

        let mut team = self.team_repo.find_by_id(id).await?.ok_or(TeamRepoError::NotFound)?;
        team.name    = name;
        let team     = self.team_repo.update(&team).await?;
        Ok(team)
    }
}
