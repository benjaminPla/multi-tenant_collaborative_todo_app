use crate::interface::{app_state::AppState, teams::handlers};
use axum::{
    routing::{get, patch},
    Router,
};

pub fn teams_router() -> Router<AppState> {
    Router::new()
        .route("/",                       get(handlers::list).post(handlers::create))
        .route("/{id}",                   get(handlers::get).patch(handlers::update).delete(handlers::delete))
        .route("/{id}/members",           get(handlers::list_members).post(handlers::add_member))
        .route("/{id}/members/{user_id}", patch(handlers::update_member).delete(handlers::remove_member))
}
