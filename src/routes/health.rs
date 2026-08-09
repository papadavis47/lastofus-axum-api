use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    data: HealthData,
}

#[derive(Serialize)]
struct HealthData {
    status: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        data: HealthData { status: "ok" },
    })
}
