use crate::interface::{app_state::AppState, boards::handlers};
use axum::{routing::get, Router};

pub fn boards_router() -> Router<AppState> {
    Router::new()
        .route("/",           get(handlers::list).post(handlers::create))
        .route("/{board_id}", get(handlers::get).patch(handlers::update).delete(handlers::delete))
}
