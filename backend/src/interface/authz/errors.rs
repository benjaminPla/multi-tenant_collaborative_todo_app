use crate::application::authz::errors::AuthzError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for AuthzError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthzError::Forbidden => StatusCode::FORBIDDEN,
            AuthzError::NotFound  => StatusCode::NOT_FOUND,
            AuthzError::Pip(_)    => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "internal error".to_owned(),
            _                                 => self.to_string(),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
