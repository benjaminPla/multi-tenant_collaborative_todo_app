use crate::{
    application::boards::errors::BoardAppError,
    domain::boards::ports::BoardRepoError,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for BoardAppError {
    fn into_response(self) -> Response {
        let status = match &self {
            BoardAppError::Domain(_)                           => StatusCode::UNPROCESSABLE_ENTITY,
            BoardAppError::Repo(BoardRepoError::NotFound)      => StatusCode::NOT_FOUND,
            BoardAppError::Repo(BoardRepoError::Unexpected(_)) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "internal error".to_owned(),
            _                                 => self.to_string(),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
