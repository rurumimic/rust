use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Injected error by no parameter: {message}")]
    Injected { message: String },

    #[error("Unexpected server error: {0}")]
    Unexpected(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Injected { .. } => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            AppError::Unexpected(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}
