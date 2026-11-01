use crate::{
    application::todos::errors::TodoAppError,
    domain::{boards::ports::BoardRepoError, todos::ports::TodoRepoError},
    interface::errors::respond,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

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
        respond(status, &self)
    }
}
