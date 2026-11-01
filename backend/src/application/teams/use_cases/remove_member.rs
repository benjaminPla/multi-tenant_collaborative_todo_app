use crate::{
    application::teams::{access::require_role, errors::TeamAppError},
    domain::{
        teams::{ports::TeamRepo, TeamId, TeamRole},
        users::{User, UserId},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct RemoveMemberInput {
    pub actor:   User,
    pub team_id: Uuid,
    pub user_id: Uuid,
}

pub struct RemoveMemberUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl RemoveMemberUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: RemoveMemberInput) -> Result<(), TeamAppError> {
        let team_id = TeamId::from(input.team_id);
        let user_id = UserId::from(input.user_id);
        let min     = match input.actor.id == user_id {
            true  => TeamRole::Member,
            false => TeamRole::Owner,
        };
        require_role(&*self.team_repo, &input.actor, team_id, min).await?;
        self.team_repo.remove_member(team_id, user_id).await?;
        Ok(())
    }
}
