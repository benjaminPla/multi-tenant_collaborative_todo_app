use crate::domain::{
    teams::{TeamId, TeamRole},
    users::UserId,
    BoxError,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
#[error("policy information failure")]
pub struct PipError(#[source] pub BoxError);

#[async_trait]
pub trait Pip: Send + Sync {
    async fn team_role(&self, team_id: TeamId, user_id: UserId) -> Result<Option<TeamRole>, PipError>;
}
