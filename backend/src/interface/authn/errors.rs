use crate::{
    application::authn::errors::AuthnAppError,
    interface::errors::respond,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for AuthnAppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthnAppError::InvalidCredentials
            | AuthnAppError::Unauthenticated => StatusCode::UNAUTHORIZED,
            AuthnAppError::Hasher(_)
            | AuthnAppError::SessionRepo(_)
            | AuthnAppError::UserRepo(_)     => StatusCode::INTERNAL_SERVER_ERROR,
        };
        respond(status, &self)
    }
}
