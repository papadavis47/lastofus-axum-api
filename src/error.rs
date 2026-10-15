use axum::{
    Json,
    extract::rejection::{PathRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

/// Every error a handler can return. Each variant renders as the structured
/// `{"error": {status, code, message}}` body described in the project plan.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{message}")]
    NotFound { code: &'static str, message: String },

    #[error("{message}")]
    BadRequest { code: &'static str, message: String },

    #[error("An unexpected error occurred.")]
    Internal,
}

impl AppError {
    pub fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self::NotFound {
            code,
            message: message.into(),
        }
    }

    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self::BadRequest {
            code,
            message: message.into(),
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::NotFound { code, .. } | Self::BadRequest { code, .. } => code,
            Self::Internal => "internal_error",
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    status: u16,
    code: &'static str,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let body = ErrorResponse {
            error: ErrorBody {
                status: status.as_u16(),
                code: self.code(),
                message: self.to_string(),
            },
        };

        (status, Json(body)).into_response()
    }
}

impl From<PathRejection> for AppError {
    fn from(rejection: PathRejection) -> Self {
        Self::bad_request("invalid_path_parameter", rejection.body_text())
    }
}

impl From<QueryRejection> for AppError {
    fn from(rejection: QueryRejection) -> Self {
        Self::bad_request("invalid_query_parameter", rejection.body_text())
    }
}

pub async fn not_found() -> AppError {
    AppError::not_found("route_not_found", "The requested route was not found.")
}
