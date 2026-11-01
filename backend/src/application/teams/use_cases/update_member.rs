use crate::{
    application::teams::errors::TeamAppError,
    domain::{
        teams::{ports::TeamRepo, Member, TeamId, TeamRole},
        users::UserId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateMemberInput {
    pub role:    String,
    pub team_id: Uuid,
    pub user_id: Uuid,
}

pub struct UpdateMemberUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl UpdateMemberUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: UpdateMemberInput) -> Result<Member, TeamAppError> {
        let role    = TeamRole::parse(&input.role)?;
        let team_id = TeamId::from(input.team_id);
        let user_id = UserId::from(input.user_id);
        let member  = self.team_repo.update_member_role(team_id, user_id, role).await?;
        Ok(member)
    }
}
