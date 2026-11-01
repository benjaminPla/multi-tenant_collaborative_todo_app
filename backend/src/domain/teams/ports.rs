use crate::domain::{
    teams::{Team, TeamId, TeamRole},
    users::UserId,
    BoxError,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum TeamRepoError {
    #[error("team not found")]
    NotFound,
    #[error(transparent)]
    Unexpected(BoxError),
}

#[async_trait]
pub trait TeamRepo: Send + Sync {
    async fn create(&self, team: &Team, owner_id: UserId)  -> Result<Team, TeamRepoError>;
    async fn delete(&self, id: TeamId)                     -> Result<(), TeamRepoError>;
    async fn find_by_id(&self, id: TeamId)                 -> Result<Option<Team>, TeamRepoError>;
    async fn find_role(&self, id: TeamId, user_id: UserId) -> Result<Option<TeamRole>, TeamRepoError>;
    async fn list(&self)                                   -> Result<Vec<Team>, TeamRepoError>;
    async fn list_by_user_id(&self, user_id: UserId)       -> Result<Vec<Team>, TeamRepoError>;
    async fn update(&self, team: &Team)                    -> Result<Team, TeamRepoError>;
}
