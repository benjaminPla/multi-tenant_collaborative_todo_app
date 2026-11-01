use crate::{
    application::teams::errors::TeamAppError,
    domain::{
        teams::{
            ports::{TeamRepo, TeamRepoError},
            TeamId,
            TeamRole,
        },
        users::User,
    },
};

pub async fn require_role(
    team_repo: &dyn TeamRepo,
    actor:     &User,
    team_id:   TeamId,
    min:       TeamRole,
) -> Result<(), TeamAppError> {
    if actor.is_admin {
        return Ok(());
    }
    match team_repo.find_role(team_id, actor.id).await? {
        Some(role) if role >= min => Ok(()),
        Some(_)                   => Err(TeamAppError::Forbidden),
        None                      => Err(TeamRepoError::NotFound.into()),
    }
}
