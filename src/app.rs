use std::time::Duration;

use axum::{
    Router,
    http::{Method, header},
    routing::get,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::{
    error::not_found,
    routes::{episodes, health::health, seasons},
    state::AppState,
};

pub fn router(state: AppState) -> Router {
    let api_v1 = Router::new()
        .route("/seasons", get(seasons::list_seasons))
        .route("/seasons/{season_number}", get(seasons::get_season))
        .route(
            "/seasons/{season_number}/episodes",
            get(seasons::list_season_episodes),
        )
        .route("/episodes", get(episodes::list_episodes))
        .route("/episodes/{id}", get(episodes::get_episode));

    Router::new()
        .route("/health", get(health))
        .nest("/api/v1", api_v1)
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
