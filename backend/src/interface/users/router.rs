use crate::interface::{app_state::AppState, users::handlers};
use axum::{routing::get, Router};

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/",     get(handlers::list).post(handlers::create))
        .route("/{id}", get(handlers::get).patch(handlers::update).delete(handlers::delete))
}
