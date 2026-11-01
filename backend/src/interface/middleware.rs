use crate::interface::errors::json_error;
use axum::{http::StatusCode, response::Response, BoxError};
use std::any::Any;
use tower::load_shed::error::Overloaded;

pub async fn overloaded(err: BoxError) -> Response {
    match err.is::<Overloaded>() {
        true  => json_error(StatusCode::SERVICE_UNAVAILABLE, "server is busy, retry shortly"),
        false => json_error(StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
    }
}

pub fn panicked(panic: Box<dyn Any + Send + 'static>) -> Response {
    let message = panic
        .downcast_ref::<&str>()
        .map(|message| message.to_string())
        .or_else(|| panic.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    tracing::error!(panic = %message, "handler panicked");
    json_error(StatusCode::INTERNAL_SERVER_ERROR, "internal error")
}
