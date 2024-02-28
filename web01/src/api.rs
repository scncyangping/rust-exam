pub mod chat;
pub mod jwt;
pub mod users;

use crate::api::jwt::AuthError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn create_user() {}

pub enum ApiError {
    Auth(AuthError),
    Internal(anyhow::Error),
}

impl From<AuthError> for ApiError {
    fn from(value: AuthError) -> Self {
        ApiError::Auth(value)
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        ApiError::Internal(value.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
