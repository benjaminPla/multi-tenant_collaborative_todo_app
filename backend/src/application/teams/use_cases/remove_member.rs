use crate::{
    application::teams::errors::TeamAppError,
    domain::{
        teams::{ports::TeamRepo, TeamId},
        users::UserId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct RemoveMemberInput {
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
        self.team_repo.remove_member(team_id, user_id).await?;
        Ok(())
    }
}
