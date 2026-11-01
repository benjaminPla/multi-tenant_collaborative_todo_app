use crate::application::authn::errors::AuthnAppError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for AuthnAppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthnAppError::Forbidden         => StatusCode::FORBIDDEN,
            AuthnAppError::InvalidCredentials
            | AuthnAppError::Unauthenticated => StatusCode::UNAUTHORIZED,
            AuthnAppError::Hasher(_)
            | AuthnAppError::SessionRepo(_)
            | AuthnAppError::UserRepo(_)     => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "internal error".to_owned(),
            _                                 => self.to_string(),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
