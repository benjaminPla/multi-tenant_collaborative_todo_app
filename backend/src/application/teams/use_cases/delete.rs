use crate::{
    application::teams::{access::require_role, errors::TeamAppError},
    domain::{
        teams::{ports::TeamRepo, TeamId, TeamRole},
        users::User,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteTeamInput {
    pub actor: User,
    pub id:    Uuid,
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
        require_role(&*self.team_repo, &input.actor, id, TeamRole::Owner).await?;
        self.team_repo.delete(id).await?;
        Ok(())
    }
}
