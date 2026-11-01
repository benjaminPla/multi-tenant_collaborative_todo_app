use crate::{
    application::teams::{access::require_role, errors::TeamAppError},
    domain::{
        teams::{ports::TeamRepo, Member, TeamId, TeamRole},
        users::User,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct ListMembersInput {
    pub actor:   User,
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
        require_role(&*self.team_repo, &input.actor, team_id, TeamRole::Member).await?;
        let members = self.team_repo.list_members(team_id).await?;
        Ok(members)
    }
}
