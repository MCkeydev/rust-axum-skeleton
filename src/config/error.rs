use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use log::error;

// Re-define Result type to handle AppError
pub type ApiResult<T> = Result<T, AppError>;

// All app related errors (TODO: might want to improve this to catch all errors)
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Something went went wrong: {0}")]
    InternalServerError(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // Log to error channel whenever a response is returned
        error!("{}", self.to_string());

        (status_code, self.to_string()).into_response()
    }
}
