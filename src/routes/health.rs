use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    data: HealthData,
}

#[derive(Serialize, ToSchema)]
struct HealthData {
    #[schema(example = "ok")]
    status: &'static str,
}

/// Health check
///
/// Reports that the server is up. It doesn't touch the database.
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses((status = OK, description = "The server is running.", body = HealthResponse)),
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        data: HealthData { status: "ok" },
    })
}
