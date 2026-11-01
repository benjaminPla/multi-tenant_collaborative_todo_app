use crate::{
    application::teams::errors::TeamAppError,
    domain::{
        teams::{ports::TeamRepo, Member, TeamId, TeamRole},
        users::Email,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct AddMemberInput {
    pub email:   String,
    pub role:    Option<String>,
    pub team_id: Uuid,
}

pub struct AddMemberUseCase {
    team_repo: Arc<dyn TeamRepo>,
}

impl AddMemberUseCase {
    pub fn new(team_repo: Arc<dyn TeamRepo>) -> Self {
        Self { team_repo }
    }

    pub async fn execute(&self, input: AddMemberInput) -> Result<Member, TeamAppError> {
        let email   = Email::parse(&input.email)?;
        let role    = input.role.as_deref().map(TeamRole::parse).transpose()?.unwrap_or(TeamRole::Member);
        let team_id = TeamId::from(input.team_id);
        let member  = self.team_repo.add_member(team_id, &email, role).await?;
        Ok(member)
    }
}
