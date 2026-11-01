use crate::{
    application::teams::errors::TeamAppError,
    domain::teams::{ports::TeamRepo, TeamId},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteTeamInput {
    pub id: Uuid,
}

pub struct DeleteTeamUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl DeleteTeamUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: DeleteTeamInput) -> Result<(), TeamAppError> {
        let id = TeamId::from(input.id);
        self.team_repo.delete(id).await?;
        Ok(())
    }
}
