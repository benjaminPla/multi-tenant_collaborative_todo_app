use crate::{
    application::users::errors::UserAppError,
    domain::users::ports::UserRepoError,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for UserAppError {
    fn into_response(self) -> Response {
        let status = match &self {
            UserAppError::Domain(_)                         => StatusCode::UNPROCESSABLE_ENTITY,
            UserAppError::Repo(UserRepoError::EmailTaken)   => StatusCode::CONFLICT,
            UserAppError::Repo(UserRepoError::NotFound)     => StatusCode::NOT_FOUND,
            UserAppError::Hasher(_)
            | UserAppError::Repo(UserRepoError::Unexpected(_))
            | UserAppError::Session(_)                     => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "internal error".to_owned(),
            _                                 => self.to_string(),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
