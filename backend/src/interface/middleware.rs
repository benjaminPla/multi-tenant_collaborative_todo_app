use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    BoxError,
    Json,
};
use serde_json::json;
use std::any::Any;
use tower::load_shed::error::Overloaded;

pub async fn overloaded(err: BoxError) -> Response {
    match err.is::<Overloaded>() {
        true  => error(StatusCode::SERVICE_UNAVAILABLE, "server is busy, retry shortly"),
        false => error(StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
    }
}

pub fn panicked(_: Box<dyn Any + Send + 'static>) -> Response {
    error(StatusCode::INTERNAL_SERVER_ERROR, "internal error")
}

fn error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "error": message }))).into_response()
}
