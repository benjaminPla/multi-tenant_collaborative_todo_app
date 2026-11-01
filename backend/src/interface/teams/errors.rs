use crate::{
    application::teams::errors::TeamAppError,
    domain::teams::ports::TeamRepoError,
    interface::errors::respond,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for TeamAppError {
    fn into_response(self) -> Response {
        let status = match &self {
            TeamAppError::Domain(_)
            | TeamAppError::User(_)                           => StatusCode::UNPROCESSABLE_ENTITY,
            TeamAppError::Repo(TeamRepoError::AlreadyMember)
            | TeamAppError::Repo(TeamRepoError::LastOwner)    => StatusCode::CONFLICT,
            TeamAppError::Repo(TeamRepoError::MemberNotFound)
            | TeamAppError::Repo(TeamRepoError::NotFound)
            | TeamAppError::Repo(TeamRepoError::UserNotFound) => StatusCode::NOT_FOUND,
            TeamAppError::Repo(TeamRepoError::Unexpected(_))  => StatusCode::INTERNAL_SERVER_ERROR,
        };
        respond(status, &self)
    }
}
