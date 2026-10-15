use axum::{Json, extract::State};
use serde::Deserialize;

use crate::{
    error::AppError,
    extract::{ApiPath, ApiQuery},
    models::Episode,
    response::{ApiResponse, Paginated, Pagination},
    state::AppState,
};

#[derive(Debug, Default, Deserialize)]
pub struct EpisodeFilter {
    /// Season number, e.g. `?season=1`.
    season: Option<u32>,
}

pub async fn list_episodes(
    State(state): State<AppState>,
    ApiQuery(filter): ApiQuery<EpisodeFilter>,
    ApiQuery(pagination): ApiQuery<Pagination>,
) -> Result<Json<Paginated<Episode>>, AppError> {
    let pagination = pagination.validate()?;

    // An unknown season matches nothing, so it yields an empty page, not a 404.
    let season_id = filter.season.map(|number| {
        state
            .seasons
            .iter()
            .find(|season| season.number == number)
            .map(|season| season.id)
    });

    let episodes: Vec<Episode> = state
        .episodes
        .iter()
        .filter(|episode| season_id.is_none_or(|id| id == Some(episode.season_id)))
        .cloned()
        .collect();

    Ok(Json(pagination.paginate(&episodes)))
}

pub async fn get_episode(
    State(state): State<AppState>,
    ApiPath(id): ApiPath<u32>,
) -> Result<Json<ApiResponse<Episode>>, AppError> {
    state
        .episodes
        .iter()
        .find(|episode| episode.id == id)
        .map(|episode| Json(ApiResponse::new(episode.clone())))
        .ok_or_else(|| {
            AppError::not_found("episode_not_found", format!("Episode {id} was not found."))
        })
}
