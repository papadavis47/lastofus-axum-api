use std::time::Duration;

use axum::{
    Json,
    http::{Method, header},
    response::Html,
    routing::get,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    error::not_found,
    openapi::{ApiDoc, DOCS_HTML},
    routes::{characters, episodes, health, seasons},
    state::AppState,
};

pub fn router(state: AppState) -> axum::Router {
    // `routes!` registers each handler with Axum and adds its `#[utoipa::path]`
    // to the spec, so the two can't drift apart. Each call takes one path.
    let api_v1 = OpenApiRouter::new()
        .routes(routes!(seasons::list_seasons))
        .routes(routes!(seasons::get_season))
        .routes(routes!(seasons::list_season_episodes))
        .routes(routes!(episodes::list_episodes))
        .routes(routes!(episodes::get_episode))
        .routes(routes!(episodes::list_episode_characters))
        .routes(routes!(characters::list_characters))
        .routes(routes!(characters::get_character))
        .routes(routes!(characters::list_character_episodes));

    let (router, spec) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health::health))
        .nest("/api/v1", api_v1)
        .split_for_parts();

    router
        // Documentation routes stay out of the spec they serve.
        .route(
            "/api/v1/openapi.json",
            get(move || async move { Json(spec) }),
        )
        .route("/api/v1/docs", get(|| async { Html(DOCS_HTML) }))
        .fallback(not_found)
        .layer(cors())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// A public, read-only API: any origin may `GET` it from a browser. No
/// credentials are involved, so the wildcard origin is safe. The layer answers
/// preflight `OPTIONS` requests itself, before routing.
fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::HEAD])
        .allow_headers([header::ACCEPT, header::CONTENT_TYPE])
        .max_age(Duration::from_secs(24 * 60 * 60))
}
