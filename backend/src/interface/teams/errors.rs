use crate::{
    application::teams::errors::TeamAppError,
    domain::teams::ports::TeamRepoError,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for TeamAppError {
    fn into_response(self) -> Response {
        let status = match &self {
            TeamAppError::Domain(_)
            | TeamAppError::User(_)                           => StatusCode::UNPROCESSABLE_ENTITY,
            TeamAppError::Forbidden                           => StatusCode::FORBIDDEN,
            TeamAppError::Repo(TeamRepoError::AlreadyMember)
            | TeamAppError::Repo(TeamRepoError::LastOwner)    => StatusCode::CONFLICT,
            TeamAppError::Repo(TeamRepoError::MemberNotFound)
            | TeamAppError::Repo(TeamRepoError::NotFound)
            | TeamAppError::Repo(TeamRepoError::UserNotFound) => StatusCode::NOT_FOUND,
            TeamAppError::Repo(TeamRepoError::Unexpected(_))  => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "internal error".to_owned(),
            _                                 => self.to_string(),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
