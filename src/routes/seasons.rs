use axum::{Json, extract::State};
use turso::Connection;

use crate::{
    db,
    error::AppError,
    extract::{ApiPath, ApiQuery},
    models::{Episode, Season},
    response::{ApiResponse, Paginated, Pagination},
    routes::episodes,
    state::AppState,
};

pub async fn list_seasons(
    State(state): State<AppState>,
    ApiQuery(pagination): ApiQuery<Pagination>,
) -> Result<Json<Paginated<Season>>, AppError> {
    let pagination = pagination.validate()?;
    let conn = state.db.connect().await?;

    let total = db::count(&conn, "SELECT COUNT(*) FROM seasons", ()).await?;
    let seasons = db::query_all(
        &conn,
        &format!(
            "SELECT {} FROM {} ORDER BY s.number LIMIT ?1 OFFSET ?2",
            Season::COLUMNS,
            Season::FROM
        ),
        (pagination.limit, pagination.offset()),
        Season::from_row,
    )
    .await?;

    Ok(Json(pagination.page(seasons, total)))
}

pub async fn get_season(
    State(state): State<AppState>,
    ApiPath(season_number): ApiPath<u32>,
) -> Result<Json<ApiResponse<Season>>, AppError> {
    let conn = state.db.connect().await?;
    let season = find_season(&conn, season_number).await?;
    Ok(Json(ApiResponse::new(season)))
}

pub async fn list_season_episodes(
    State(state): State<AppState>,
    ApiPath(season_number): ApiPath<u32>,
    ApiQuery(pagination): ApiQuery<Pagination>,
) -> Result<Json<Paginated<Episode>>, AppError> {
    let pagination = pagination.validate()?;
    let conn = state.db.connect().await?;

    find_season(&conn, season_number).await?;
    let page = episodes::fetch_page(&conn, Some(season_number), pagination).await?;

    Ok(Json(page))
}

async fn find_season(conn: &Connection, season_number: u32) -> Result<Season, AppError> {
    db::query_optional(
        conn,
        &format!(
            "SELECT {} FROM {} WHERE s.number = ?1",
            Season::COLUMNS,
            Season::FROM
        ),
        (season_number,),
        Season::from_row,
    )
    .await?
    .ok_or_else(|| {
        AppError::not_found(
            "season_not_found",
            format!("Season {season_number} was not found."),
        )
    })
}
