use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

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
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
