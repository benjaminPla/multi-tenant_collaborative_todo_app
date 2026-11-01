use crate::interface::{app_state::AppState, teams::handlers};
use axum::{routing::get, Router};

pub fn teams_router() -> Router<AppState> {
    Router::new()
        .route("/",     get(handlers::list).post(handlers::create))
        .route("/{id}", get(handlers::get).patch(handlers::update).delete(handlers::delete))
}
