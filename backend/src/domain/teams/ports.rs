use crate::domain::{
    teams::{Member, Team, TeamId, TeamRole},
    users::{Email, UserId},
    BoxError,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum TeamRepoError {
    #[error("user is already a member")]
    AlreadyMember,
    #[error("team must keep at least one owner")]
    LastOwner,
    #[error("member not found")]
    MemberNotFound,
    #[error("team not found")]
    NotFound,
    #[error(transparent)]
    Unexpected(BoxError),
    #[error("user not found")]
    UserNotFound,
}

#[async_trait]
pub trait TeamRepo: Send + Sync {
    async fn add_member(&self, id: TeamId, email: &Email, role: TeamRole)           -> Result<Member, TeamRepoError>;
    async fn create(&self, team: &Team, owner_id: UserId)                           -> Result<Team, TeamRepoError>;
    async fn delete(&self, id: TeamId)                                              -> Result<(), TeamRepoError>;
    async fn find_by_id(&self, id: TeamId)                                          -> Result<Option<Team>, TeamRepoError>;
    async fn find_role(&self, id: TeamId, user_id: UserId)                          -> Result<Option<TeamRole>, TeamRepoError>;
    async fn list(&self)                                                            -> Result<Vec<Team>, TeamRepoError>;
    async fn list_by_user_id(&self, user_id: UserId)                                -> Result<Vec<Team>, TeamRepoError>;
    async fn list_members(&self, id: TeamId)                                        -> Result<Vec<Member>, TeamRepoError>;
    async fn remove_member(&self, id: TeamId, user_id: UserId)                      -> Result<(), TeamRepoError>;
    async fn update(&self, team: &Team)                                             -> Result<Team, TeamRepoError>;
    async fn update_member_role(&self, id: TeamId, user_id: UserId, role: TeamRole) -> Result<Member, TeamRepoError>;
}
