use crate::interface::{app_state::AppState, todos::handlers};
use axum::{routing::get, Router};

pub fn todos_router() -> Router<AppState> {
    Router::new()
        .route("/",          get(handlers::list).post(handlers::create))
        .route("/events",    get(handlers::events))
        .route("/{todo_id}", get(handlers::get).patch(handlers::update).delete(handlers::delete))
}
