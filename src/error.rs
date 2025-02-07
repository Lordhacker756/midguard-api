use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub status_code: Option<StatusCode>,
    pub message: String,
}

impl AppError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            status_code: None,
            message: message.into(),
        }
    }

    pub fn with_status(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }
}

// Add common error conversions
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::new(err.to_string()).with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::new(err.to_string()).with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error [{}]: {}",
            self.status_code
                .map_or("Unknown".to_string(), |s| s.to_string()),
            self.message
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self
            .status_code
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(json!({
            "statusCode": status.as_u16(),
            "message": self.message,
        }));

        (status, body).into_response()
    }
}
