use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;
use tracing::error;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limited: try again in {0} seconds")]
    RateLimited(u64),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Inference error: {0}")]
    Inference(String),

    #[error("File too large: max {0} bytes")]
    FileTooLarge(u64),

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("External API error: {0}")]
    ExternalApi(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Validation(_) | Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::RateLimited(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::FileTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            Self::UnsupportedFileType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::Database(_) | Self::Inference(_) | Self::ExternalApi(_) => {
                StatusCode::BAD_GATEWAY
            }
            Self::Internal(_) | Self::Cache(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "NOT_FOUND",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::Conflict(_) => "CONFLICT",
            Self::RateLimited(_) => "RATE_LIMITED",
            Self::Internal(_) => "INTERNAL_ERROR",
            Self::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Cache(_) => "CACHE_ERROR",
            Self::Inference(_) => "INFERENCE_ERROR",
            Self::FileTooLarge(_) => "FILE_TOO_LARGE",
            Self::UnsupportedFileType(_) => "UNSUPPORTED_FILE_TYPE",
            Self::ExternalApi(_) => "EXTERNAL_API_ERROR",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let request_id = String::new();

        let body = json!({
            "error": {
                "code": self.error_code(),
                "message": self.to_string(),
                "request_id": request_id,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        });

        if status.is_server_error() {
            error!(error = %self, status = %status, "Server error");
        }

        (status, Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::NotFound("Resource not found".into()),
            sqlx::Error::Database(ref d) if d.constraint().is_some() => {
                Self::Conflict(format!("Constraint violation: {}", d.constraint().unwrap()))
            }
            _ => {
                error!(error = %e, "Database error");
                Self::Database(e.to_string())
            }
        }
    }
}

impl From<eyre::ErrReport> for AppError {
    fn from(e: eyre::ErrReport) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::Validation(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(_: argon2::password_hash::Error) -> Self {
        Self::Internal("Password hashing failed".into())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                Self::Unauthorized("Token expired".into())
            }
            jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::InvalidToken => {
                Self::Unauthorized("Invalid token".into())
            }
            _ => Self::Internal(format!("JWT error: {e}")),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(e: validator::ValidationErrors) -> Self {
        Self::Validation(e.to_string())
    }
}
