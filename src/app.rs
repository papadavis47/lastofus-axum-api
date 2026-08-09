use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::{error::not_found, routes::health::health};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use super::router;

    #[tokio::test]
    async fn health_returns_ok_without_binding_a_port() {
        let response = router()
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json_body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json_body, json!({ "data": { "status": "ok" } }));
    }

    #[tokio::test]
    async fn unknown_routes_return_a_structured_error() {
        let response = router()
            .oneshot(Request::get("/missing").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json_body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json_body,
            json!({
                "error": {
                    "status": 404,
                    "code": "route_not_found",
                    "message": "The requested route was not found."
                }
            })
        );
    }
}
