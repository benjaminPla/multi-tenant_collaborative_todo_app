use crate::{
    application::users::errors::UserAppError,
    domain::users::ports::UserRepoError,
    interface::errors::respond,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

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
        respond(status, &self)
    }
}
