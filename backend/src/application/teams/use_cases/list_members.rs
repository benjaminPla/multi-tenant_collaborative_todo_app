use crate::{
    application::teams::errors::TeamAppError,
    domain::teams::{ports::TeamRepo, Member, TeamId},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct ListMembersInput {
    pub team_id: Uuid,
}

pub struct ListMembersUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl ListMembersUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: ListMembersInput) -> Result<Vec<Member>, TeamAppError> {
        let team_id = TeamId::from(input.team_id);
        let members = self.team_repo.list_members(team_id).await?;
        Ok(members)
    }
}
