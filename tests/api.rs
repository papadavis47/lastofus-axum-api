//! Black-box tests that send requests straight to the router; no port is bound.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use lastofus_axum_api::{app::router, state::AppState};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn get(uri: &str) -> (StatusCode, Value) {
    let response = router(AppState::from_fixtures())
        .oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&body).unwrap())
}

fn assert_error(body: &Value, status: u16, code: &str) {
    assert_eq!(body["error"]["status"], status);
    assert_eq!(body["error"]["code"], code);
    assert!(body["error"]["message"].is_string());
}

#[tokio::test]
async fn health_returns_ok() {
    let (status, body) = get("/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "data": { "status": "ok" } }));
}

#[tokio::test]
async fn unknown_routes_return_a_structured_error() {
    for uri in ["/missing", "/api/v1/missing"] {
        let (status, body) = get(uri).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(
            body,
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

#[tokio::test]
async fn lists_seasons_with_pagination_meta() {
    let (status, body) = get("/api/v1/seasons").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"].as_array().unwrap().len(), 2);
    assert_eq!(
        body["meta"],
        json!({ "page": 1, "limit": 20, "total": 2, "total_pages": 1 })
    );
}

#[tokio::test]
async fn gets_a_season_by_number() {
    let (status, body) = get("/api/v1/seasons/1").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["number"], 1);
    assert_eq!(body["data"]["release_year"], 2023);
}

#[tokio::test]
async fn unknown_season_returns_404() {
    let (status, body) = get("/api/v1/seasons/99").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_error(&body, 404, "season_not_found");
    assert_eq!(body["error"]["message"], "Season 99 was not found.");

    let (status, body) = get("/api/v1/seasons/99/episodes").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_error(&body, 404, "season_not_found");
}

#[tokio::test]
async fn lists_a_seasons_episodes() {
    let (status, body) = get("/api/v1/seasons/1/episodes").await;

    assert_eq!(status, StatusCode::OK);
    let episodes = body["data"].as_array().unwrap();
    assert_eq!(episodes.len(), 9);
    assert!(episodes.iter().all(|episode| episode["season_id"] == 1));
}

#[tokio::test]
async fn gets_an_episode_by_id() {
    let (status, body) = get("/api/v1/episodes/1").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["title"], "When You're Lost in the Darkness");
    assert_eq!(body["data"]["release_date"], "2023-01-15");
    // Unknown values are null, never empty strings.
    assert_eq!(body["data"]["image_url"], Value::Null);
}

#[tokio::test]
async fn unknown_episode_returns_404() {
    let (status, body) = get("/api/v1/episodes/99").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_error(&body, 404, "episode_not_found");
    assert_eq!(body["error"]["message"], "Episode 99 was not found.");
}

#[tokio::test]
async fn non_numeric_episode_id_returns_400() {
    let (status, body) = get("/api/v1/episodes/abc").await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_error(&body, 400, "invalid_path_parameter");
}

#[tokio::test]
async fn filters_episodes_by_season() {
    let (_, body) = get("/api/v1/episodes?season=1").await;
    assert_eq!(body["meta"]["total"], 9);

    let (status, body) = get("/api/v1/episodes?season=99").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"], json!([]));
    assert_eq!(body["meta"]["total"], 0);
    assert_eq!(body["meta"]["total_pages"], 0);
}

#[tokio::test]
async fn paginates_episodes() {
    let (status, body) = get("/api/v1/episodes?page=2&limit=4").await;

    assert_eq!(status, StatusCode::OK);
    let ids: Vec<u64> = body["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|episode| episode["id"].as_u64().unwrap())
        .collect();
    assert_eq!(ids, [5, 6, 7, 8]);
    assert_eq!(
        body["meta"],
        json!({ "page": 2, "limit": 4, "total": 9, "total_pages": 3 })
    );

    let (status, body) = get("/api/v1/episodes?page=10").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"], json!([]));
}

#[tokio::test]
async fn invalid_pagination_returns_400() {
    for uri in [
        "/api/v1/episodes?page=0",
        "/api/v1/episodes?limit=0",
        "/api/v1/episodes?limit=101",
        "/api/v1/seasons?page=0",
        "/api/v1/seasons/1/episodes?limit=500",
    ] {
        let (status, body) = get(uri).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}");
        assert_error(&body, 400, "invalid_pagination");
    }
}

#[tokio::test]
async fn malformed_query_returns_400() {
    for uri in ["/api/v1/episodes?season=one", "/api/v1/episodes?page=-1"] {
        let (status, body) = get(uri).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}");
        assert_error(&body, 400, "invalid_query_parameter");
    }
}
