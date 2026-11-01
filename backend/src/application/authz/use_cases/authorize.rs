use crate::{
    application::authz::errors::AuthzError,
    domain::{
        authz::{pdp::decide, ports::Pip, Action, Decision, Resource, Scope, Subject},
        users::User,
    },
};
use std::sync::Arc;

pub struct AuthorizeInput {
    pub action:   Action,
    pub actor:    User,
    pub resource: Resource,
}

pub struct AuthorizeUseCase {
    pip: Arc<dyn Pip>,
}

impl AuthorizeUseCase {
    pub fn new(pip: Arc<dyn Pip>) -> Self {
        Self { pip }
    }

    pub async fn execute(&self, input: AuthorizeInput) -> Result<Scope, AuthzError> {
        let team_role = match (input.actor.is_admin, input.resource.team_id()) {
            (false, Some(team_id)) => self.pip.team_role(team_id, input.actor.id).await?,
            _                      => None,
        };
        let subject    = Subject { id: input.actor.id, is_admin: input.actor.is_admin, team_role };
        let non_member = input.resource.team_id().is_some() && subject.team_role.is_none();

        match decide(&subject, input.action, &input.resource) {
            Decision::Permit(scope)      => Ok(scope),
            Decision::Deny if non_member => Err(AuthzError::NotFound),
            Decision::Deny               => Err(AuthzError::Forbidden),
        }
    }
}
