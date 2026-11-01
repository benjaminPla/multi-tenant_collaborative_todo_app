use crate::{
    application::boards::errors::BoardAppError,
    domain::boards::ports::BoardRepoError,
    interface::errors::respond,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for BoardAppError {
    fn into_response(self) -> Response {
        let status = match &self {
            BoardAppError::Domain(_)                           => StatusCode::UNPROCESSABLE_ENTITY,
            BoardAppError::Repo(BoardRepoError::NotFound)      => StatusCode::NOT_FOUND,
            BoardAppError::Repo(BoardRepoError::Unexpected(_)) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        respond(status, &self)
    }
}
