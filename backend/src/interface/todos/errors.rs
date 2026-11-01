use crate::{
    application::todos::errors::TodoAppError,
    domain::{boards::ports::BoardRepoError, todos::ports::TodoRepoError},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for TodoAppError {
    fn into_response(self) -> Response {
        let status = match &self {
            TodoAppError::Board(BoardRepoError::NotFound)
            | TodoAppError::Repo(TodoRepoError::BoardNotFound)
            | TodoAppError::Repo(TodoRepoError::NotFound)          => StatusCode::NOT_FOUND,
            TodoAppError::Domain(_)
            | TodoAppError::Repo(TodoRepoError::AssigneeNotMember) => StatusCode::UNPROCESSABLE_ENTITY,
            TodoAppError::Repo(TodoRepoError::VersionConflict)     => StatusCode::CONFLICT,
            TodoAppError::Board(BoardRepoError::Unexpected(_))
            | TodoAppError::Repo(TodoRepoError::Unexpected(_))     => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "internal error".to_owned(),
            _                                 => self.to_string(),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
