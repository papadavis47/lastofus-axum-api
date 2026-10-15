use axum::{Json, extract::State};

use crate::{
    error::AppError,
    extract::{ApiPath, ApiQuery},
    models::{Episode, Season},
    response::{ApiResponse, Paginated, Pagination},
    state::AppState,
};

pub async fn list_seasons(
    State(state): State<AppState>,
    ApiQuery(pagination): ApiQuery<Pagination>,
) -> Result<Json<Paginated<Season>>, AppError> {
    Ok(Json(pagination.validate()?.paginate(&state.seasons)))
}

pub async fn get_season(
    State(state): State<AppState>,
    ApiPath(season_number): ApiPath<u32>,
) -> Result<Json<ApiResponse<Season>>, AppError> {
    let season = find_season(&state, season_number)?;
    Ok(Json(ApiResponse::new(season.clone())))
}

pub async fn list_season_episodes(
    State(state): State<AppState>,
    ApiPath(season_number): ApiPath<u32>,
    ApiQuery(pagination): ApiQuery<Pagination>,
) -> Result<Json<Paginated<Episode>>, AppError> {
    let pagination = pagination.validate()?;
    let season = find_season(&state, season_number)?;
    let episodes: Vec<Episode> = state
        .episodes
        .iter()
        .filter(|episode| episode.season_id == season.id)
        .cloned()
        .collect();

    Ok(Json(pagination.paginate(&episodes)))
}

fn find_season(state: &AppState, season_number: u32) -> Result<&Season, AppError> {
    state
        .seasons
        .iter()
        .find(|season| season.number == season_number)
        .ok_or_else(|| {
            AppError::not_found(
                "season_not_found",
                format!("Season {season_number} was not found."),
            )
        })
}
