use crate::{
    application::authz::errors::AuthzError,
    interface::errors::respond,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for AuthzError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthzError::Forbidden => StatusCode::FORBIDDEN,
            AuthzError::NotFound  => StatusCode::NOT_FOUND,
            AuthzError::Pip(_)    => StatusCode::INTERNAL_SERVER_ERROR,
        };
        respond(status, &self)
    }
}
