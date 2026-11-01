use crate::application::auth::errors::AuthAppError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for AuthAppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthAppError::Forbidden                => StatusCode::FORBIDDEN,
            AuthAppError::InvalidCredentials
            | AuthAppError::Unauthenticated        => StatusCode::UNAUTHORIZED,
            AuthAppError::Hasher(_)
            | AuthAppError::SessionRepo(_)
            | AuthAppError::UserRepo(_)            => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "internal error".to_owned(),
            _                                 => self.to_string(),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
