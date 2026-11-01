use crate::interface::{app_state::AppState, authn::handlers};
use axum::{
    routing::{get, post},
    Router,
};

pub fn authn_router() -> Router<AppState> {
    Router::new()
        .route("/login",  post(handlers::login))
        .route("/logout", post(handlers::logout))
        .route("/me",     get(handlers::me))
}
