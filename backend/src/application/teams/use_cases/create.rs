use crate::{
    application::teams::errors::TeamAppError,
    domain::{
        teams::{ports::TeamRepo, Team, TeamName},
        users::User,
    },
};
use std::sync::Arc;

pub struct CreateTeamInput {
    pub actor: User,
    pub name:  String,
}

pub struct CreateTeamUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl CreateTeamUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: CreateTeamInput) -> Result<Team, TeamAppError> {
        let name = TeamName::parse(&input.name)?;
        let team = Team::create(name);
        let team = self.team_repo.create(&team, input.actor.id).await?;
        Ok(team)
    }
}
