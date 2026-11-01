use crate::{
    application::authn::{
        errors::AuthnAppError,
        use_cases::{AuthenticateInput, AuthenticateUseCase},
    },
    domain::users::User,
    interface::{app_state::AppState, authn::cookie::SESSION_COOKIE},
};
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::cookie::CookieJar;

pub struct CurrentUser(pub User);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AuthnAppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let jar      = CookieJar::from_headers(&parts.headers);
        let token    = jar.get(SESSION_COOKIE).ok_or(AuthnAppError::Unauthenticated)?.value().to_owned();
        let use_case = AuthenticateUseCase::new(state.session_repo.clone(), state.user_repo.clone());
        let user     = use_case.execute(AuthenticateInput { token }).await?;
        Ok(Self(user))
    }
}
