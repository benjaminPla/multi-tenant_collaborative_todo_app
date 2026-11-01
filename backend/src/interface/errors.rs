use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::error::Error;

pub fn json_error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "error": message }))).into_response()
}

pub fn respond(status: StatusCode, err: &(dyn Error + 'static)) -> Response {
    if status == StatusCode::INTERNAL_SERVER_ERROR {
        tracing::error!(error = err, "request failed");
        return json_error(status, "internal error");
    }
    json_error(status, &err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, thiserror::Error)]
    #[error("connection to db-internal:5432 refused")]
    struct Leaky;

    async fn body(response: Response) -> String {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn client_errors_keep_their_message() {
        let response = respond(StatusCode::CONFLICT, &Leaky);
        assert_eq!(response.status(), StatusCode::CONFLICT);
        assert!(body(response).await.contains("db-internal"));
    }

    #[tokio::test]
    async fn internal_errors_hide_their_cause() {
        let response = respond(StatusCode::INTERNAL_SERVER_ERROR, &Leaky);
        let body     = body(response).await;
        assert!(body.contains("internal error"));
        assert!(!body.contains("db-internal"));
    }
}
