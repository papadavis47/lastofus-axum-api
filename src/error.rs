use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    status: u16,
    code: &'static str,
    message: &'static str,
}

pub async fn not_found() -> Response {
    let status = StatusCode::NOT_FOUND;
    let body = ErrorResponse {
        error: ErrorBody {
            status: status.as_u16(),
            code: "route_not_found",
            message: "The requested route was not found.",
        },
    };

    (status, Json(body)).into_response()
}
