use crate::domain::sessions::SessionToken;
use axum_extra::extract::cookie::{Cookie, SameSite};

pub const SESSION_COOKIE: &str = "session";

pub fn clear_session_cookie() -> Cookie<'static> {
    Cookie::build(SESSION_COOKIE).path("/").build()
}

pub fn session_cookie(token: &SessionToken) -> Cookie<'static> {
    Cookie::build((SESSION_COOKIE, token.as_str().to_owned()))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax)
        .secure(true)
        .build()
}
