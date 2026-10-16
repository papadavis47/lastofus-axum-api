//! Black-box tests that send requests straight to the router; no port is bound.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use lastofus_axum_api::{
    app::router,
    db::{Db, migrations},
    state::AppState,
};
use serde_json::{Value, json};
use tower::ServiceExt;

/// A fresh in-memory database with every migration (schema + seed) applied.
async fn test_db() -> Db {
    let db = Db::in_memory().await.unwrap();
    migrations::run(&db.connect().await.unwrap()).await.unwrap();
    db
}

async fn get(uri: &str) -> (StatusCode, Value) {
    get_with(test_db().await, uri).await
}

async fn get_with(db: Db, uri: &str) -> (StatusCode, Value) {
    let response = router(AppState::new(db))
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
    assert_eq!(body["data"]["image"], Value::Null);
    // IMDb has no season ids.
    assert_eq!(
        body["data"]["external_ids"],
        json!({"tmdb": null, "imdb": null, "wikidata": "Q107549810"})
    );
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
    assert_eq!(body["data"]["runtime_minutes"], 81);
    assert_eq!(
        body["data"]["writers"],
        json!(["Craig Mazin", "Neil Druckmann"])
    );
    // Unknown values are null, never empty strings.
    assert_eq!(body["data"]["image"], Value::Null);
    assert_eq!(
        body["data"]["external_ids"],
        json!({"tmdb": null, "imdb": "tt11957006", "wikidata": "Q116169494"})
    );
    assert!(body["data"].get("image_url").is_none());
}

#[tokio::test]
async fn returns_images_with_attribution() {
    let db = test_db().await;
    let conn = db.connect().await.unwrap();
    conn.execute_batch(
        "INSERT INTO images (id, url, width, height, alt_text, source_page_url, creator, \
             license_name, license_url, attribution_text, last_verified_at) VALUES \
             (1, 'https://example.org/still.jpg', 800, 450, 'A test image', \
              'https://example.org/still', 'Jane Doe', 'CC BY 4.0', \
              'https://creativecommons.org/licenses/by/4.0/', \
              'Photo by Jane Doe, CC BY 4.0', '2026-10-16'), \
             (2, 'https://example.org/public-domain.jpg', NULL, NULL, 'Another test image', \
              'https://example.org/public-domain', NULL, 'Public domain', NULL, \
              'Public domain', '2026-10-16'); \
         UPDATE episodes SET image_id = 1, tmdb_id = 42, imdb_id = 'tt0000001' WHERE id = 3; \
         UPDATE seasons SET image_id = 2, wikidata_id = 'Q1' WHERE number = 2;",
    )
    .await
    .unwrap();

    let (status, body) = get_with(db.clone(), "/api/v1/episodes/3").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body["data"]["image"],
        json!({
            "url": "https://example.org/still.jpg",
            "width": 800,
            "height": 450,
            "alt": "A test image",
            "attribution": {
                "text": "Photo by Jane Doe, CC BY 4.0",
                "creator": "Jane Doe",
                "license": "CC BY 4.0",
                "license_url": "https://creativecommons.org/licenses/by/4.0/",
                "source_url": "https://example.org/still"
            }
        })
    );
    assert_eq!(
        body["data"]["external_ids"],
        json!({"tmdb": 42, "imdb": "tt0000001", "wikidata": "Q116178387"})
    );

    // Lists join images too, and leave episodes without one as null.
    let (_, body) = get_with(db.clone(), "/api/v1/seasons/1/episodes").await;
    let episodes = body["data"].as_array().unwrap();
    assert_eq!(episodes[2]["image"]["url"], "https://example.org/still.jpg");
    assert_eq!(episodes[3]["image"], Value::Null);

    let (_, body) = get_with(db, "/api/v1/seasons").await;
    let seasons = body["data"].as_array().unwrap();
    assert_eq!(seasons[0]["image"], Value::Null);
    assert_eq!(seasons[1]["image"]["attribution"]["creator"], Value::Null);
    assert_eq!(
        seasons[1]["image"]["attribution"]["license"],
        "Public domain"
    );
    assert_eq!(seasons[1]["external_ids"]["wikidata"], "Q1");
}

#[tokio::test]
async fn lists_season_two_episodes_in_order_with_writers() {
    let (status, body) = get("/api/v1/seasons/2/episodes").await;

    assert_eq!(status, StatusCode::OK);
    let episodes = body["data"].as_array().unwrap();
    assert_eq!(episodes.len(), 7);
    let numbers: Vec<u64> = episodes
        .iter()
        .map(|episode| episode["number_in_season"].as_u64().unwrap())
        .collect();
    assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7]);
    // Writers keep their credited order.
    assert_eq!(
        episodes[5]["writers"],
        json!(["Neil Druckmann", "Halley Gross", "Craig Mazin"])
    );
}

#[tokio::test]
async fn every_episode_has_distinct_external_ids() {
    let (_, body) = get("/api/v1/episodes?limit=100").await;
    let episodes = body["data"].as_array().unwrap();
    assert_eq!(episodes.len(), 16);

    for key in ["imdb", "wikidata"] {
        let mut ids: Vec<&str> = episodes
            .iter()
            .map(|episode| episode["external_ids"][key].as_str().unwrap())
            .collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 16, "duplicate {key} ids");
    }
    assert_eq!(episodes[15]["external_ids"]["imdb"], "tt32550887");
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
        json!({ "page": 2, "limit": 4, "total": 16, "total_pages": 4 })
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

#[tokio::test]
async fn database_failures_return_a_safe_500() {
    let db = test_db().await;
    db.connect()
        .await
        .unwrap()
        .execute("DROP TABLE episode_writers", ())
        .await
        .unwrap();

    let (status, body) = get_with(db, "/api/v1/episodes").await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        body,
        json!({
            "error": {
                "status": 500,
                "code": "internal_error",
                "message": "An unexpected error occurred."
            }
        })
    );
}

#[tokio::test]
async fn migrations_are_idempotent() {
    let db = test_db().await;
    let conn = db.connect().await.unwrap();

    assert_eq!(migrations::run(&conn).await.unwrap(), Vec::<i64>::new());
    assert!(migrations::pending(&conn).await.unwrap().is_empty());
}

#[tokio::test]
async fn pending_migrations_are_reported_without_writing() {
    let db = Db::in_memory().await.unwrap();
    let conn = db.connect().await.unwrap();

    assert_eq!(migrations::pending(&conn).await.unwrap(), [1, 2, 3, 4]);

    // `pending` must not create `_migrations`: servers call it on read-only replicas.
    let mut rows = conn
        .query(
            "SELECT COUNT(*) FROM sqlite_schema WHERE name = '_migrations'",
            (),
        )
        .await
        .unwrap();
    let tables: i64 = rows.next().await.unwrap().unwrap().get(0).unwrap();
    assert_eq!(tables, 0);
}
