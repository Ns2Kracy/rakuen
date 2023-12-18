use axum::{http::StatusCode, response::IntoResponse};

use crate::common::auth::AuthError;

pub type ApiResult<T> = std::result::Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("{0}")]
    ApiError(#[from] ApiError),
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InternalServerError(String),
    #[error("{0}")]
    NotImplemented(String),
    #[error("{0}")]
    BadGateway(String),
    #[error("{0}")]
    ServiceUnavailable(String),

    #[error("{0}")]
    ValidationError(#[from] validator::ValidationErrors),
}

impl IntoResponse for ErrorKind {
    fn into_response(self) -> axum::response::Response {
        match self {
            ErrorKind::ApiError(e) => e.into_response(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::BadRequest(e) => (StatusCode::BAD_REQUEST, e),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            ApiError::Forbidden(e) => (StatusCode::FORBIDDEN, e),
            ApiError::NotFound(e) => (StatusCode::NOT_FOUND, e),
            ApiError::InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            ApiError::NotImplemented(e) => (StatusCode::NOT_IMPLEMENTED, e),
            ApiError::BadGateway(e) => (StatusCode::BAD_GATEWAY, e),
            ApiError::ServiceUnavailable(e) => (StatusCode::SERVICE_UNAVAILABLE, e),
            ApiError::ValidationError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        }
        .into_response()
    }
}

impl From<AuthError> for ApiError {
    fn from(e: AuthError) -> Self {
        match e {
            AuthError::WrongCredentials => ApiError::Unauthorized,
            AuthError::MissingCredentials => ApiError::Unauthorized,
            AuthError::TokenCreation => {
                ApiError::InternalServerError("Token creation error".to_string())
            }
            AuthError::InvalidToken => ApiError::Unauthorized,
        }
    }
}
