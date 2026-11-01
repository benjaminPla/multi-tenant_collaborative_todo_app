use crate::{
    application::authz::errors::AuthzError,
    domain::{authz::Scope, users::User},
};
use axum::{extract::FromRequestParts, http::request::Parts};

#[derive(Clone)]
pub struct Authorized {
    pub actor: User,
    pub scope: Scope,
}

impl<S: Send + Sync> FromRequestParts<S> for Authorized {
    type Rejection = AuthzError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<Self>().cloned().ok_or(AuthzError::Forbidden)
    }
}
