use crate::{
    application::{
        authn::errors::AuthnAppError,
        authz::{
            errors::AuthzError,
            use_cases::{AuthorizeInput, AuthorizeUseCase},
        },
    },
    interface::{
        app_state::AppState,
        authn::extractors::CurrentUser,
        authz::{
            authorized::Authorized,
            registry::{self, Policy},
        },
    },
};
use axum::{
    extract::{FromRequestParts, MatchedPath, RawPathParams, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
enum Rejection {
    #[error(transparent)]
    Authn(#[from] AuthnAppError),
    #[error(transparent)]
    Authz(#[from] AuthzError),
    #[error("invalid path parameter")]
    InvalidPath,
}

impl IntoResponse for Rejection {
    fn into_response(self) -> Response {
        match self {
            Rejection::Authn(err)  => err.into_response(),
            Rejection::Authz(err)  => err.into_response(),
            Rejection::InvalidPath => (StatusCode::BAD_REQUEST, Json(json!({ "error": self.to_string() }))).into_response(),
        }
    }
}

pub async fn pep(State(state): State<AppState>, request: Request, next: Next) -> Response {
    match enforce(state, request).await {
        Ok(request)    => next.run(request).await,
        Err(rejection) => rejection.into_response(),
    }
}

async fn enforce(state: AppState, request: Request) -> Result<Request, Rejection> {
    let policy = request
        .extensions()
        .get::<MatchedPath>()
        .and_then(|path| registry::lookup(request.method(), path.as_str()))
        .ok_or(AuthzError::Forbidden)?;

    let Policy::Guarded { action, loader } = policy else { return Ok(request) };

    let (mut parts, body)  = request.into_parts();
    let CurrentUser(actor) = CurrentUser::from_request_parts(&mut parts, &state).await?;
    let params             = RawPathParams::from_request_parts(&mut parts, &state).await.map_err(|_| Rejection::InvalidPath)?;
    let resource           = loader(&params).ok_or(Rejection::InvalidPath)?;

    let use_case = AuthorizeUseCase::new(state.pip);
    let scope    = use_case.execute(AuthorizeInput { action, actor: actor.clone(), resource }).await?;

    parts.extensions.insert(Authorized { actor, scope });
    Ok(Request::from_parts(parts, body))
}
