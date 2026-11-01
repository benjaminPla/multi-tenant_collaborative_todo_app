use crate::interface::{app_state::AppState, auth::handlers};
use axum::{
    routing::{get, post},
    Router,
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login",  post(handlers::login))
        .route("/logout", post(handlers::logout))
        .route("/me",     get(handlers::me))
}
