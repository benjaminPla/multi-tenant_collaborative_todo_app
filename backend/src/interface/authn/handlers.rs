use crate::{
    application::authn::{
        errors::AuthnAppError,
        use_cases::{LoginInput, LoginUseCase, LogoutInput, LogoutUseCase},
    },
    interface::{
        app_state::AppState,
        authn::{
            cookie::{clear_session_cookie, session_cookie, SESSION_COOKIE},
            dto::LoginRequest,
            extractors::CurrentUser,
        },
        users::dto::UserResponse,
    },
};
use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::cookie::CookieJar;

pub async fn login(
    State(state): State<AppState>,
    jar:          CookieJar,
    Json(body):   Json<LoginRequest>,
) -> Result<(StatusCode, CookieJar), AuthnAppError> {
    let use_case = LoginUseCase::new(state.hasher_service, state.session_repo, state.session_ttl, state.user_repo);
    let token    = use_case.execute(LoginInput { email: body.email, password: body.password }).await?;
    Ok((StatusCode::NO_CONTENT, jar.add(session_cookie(&token))))
}

pub async fn logout(
    State(state): State<AppState>,
    jar:          CookieJar,
) -> Result<(StatusCode, CookieJar), AuthnAppError> {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        let use_case = LogoutUseCase::new(state.session_repo);
        use_case.execute(LogoutInput { token: cookie.value().to_owned() }).await?;
    }
    Ok((StatusCode::NO_CONTENT, jar.remove(clear_session_cookie())))
}

pub async fn me(CurrentUser(user): CurrentUser) -> Json<UserResponse> {
    Json(user.into())
}
