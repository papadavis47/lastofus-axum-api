use std::collections::HashMap;

use axum::{Json, extract::State};
use serde::Deserialize;
use turso::Connection;

use crate::{
    db,
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
    let conn = state.db.connect().await?;

    // An unknown season matches nothing, so it yields an empty page, not a 404.
    let page = fetch_page(&conn, filter.season, pagination).await?;

    Ok(Json(page))
}

pub async fn get_episode(
    State(state): State<AppState>,
    ApiPath(id): ApiPath<u32>,
) -> Result<Json<ApiResponse<Episode>>, AppError> {
    let conn = state.db.connect().await?;

    let episode = db::query_optional(
        &conn,
        &format!(
            "SELECT {} FROM episodes e WHERE e.id = ?1",
            Episode::COLUMNS
        ),
        (id,),
        Episode::from_row,
    )
    .await?
    .ok_or_else(|| {
        AppError::not_found("episode_not_found", format!("Episode {id} was not found."))
    })?;

    let mut episodes = [episode];
    load_writers(&conn, &mut episodes).await?;
    let [episode] = episodes;

    Ok(Json(ApiResponse::new(episode)))
}

/// One page of episodes in broadcast order, optionally limited to a season
/// number. Shared with `GET /seasons/{season_number}/episodes`.
pub async fn fetch_page(
    conn: &Connection,
    season_number: Option<u32>,
    pagination: Pagination,
) -> Result<Paginated<Episode>, AppError> {
    const FILTER: &str = "FROM episodes e JOIN seasons s ON s.id = e.season_id \
                          WHERE (?1 IS NULL OR s.number = ?1)";

    let total = db::count(conn, &format!("SELECT COUNT(*) {FILTER}"), (season_number,)).await?;
    let mut episodes = db::query_all(
        conn,
        &format!(
            "SELECT {} {FILTER} ORDER BY e.overall_number LIMIT ?2 OFFSET ?3",
            Episode::COLUMNS
        ),
        (season_number, pagination.limit, pagination.offset()),
        Episode::from_row,
    )
    .await?;
    load_writers(conn, &mut episodes).await?;

    Ok(pagination.page(episodes, total))
}

/// Fills in `writers` for every episode with a single query.
async fn load_writers(conn: &Connection, episodes: &mut [Episode]) -> Result<(), AppError> {
    if episodes.is_empty() {
        return Ok(());
    }

    // Pass the ids as one JSON array parameter instead of building `IN (?, ?, ...)`.
    let ids = episodes
        .iter()
        .map(|episode| episode.id.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let rows = db::query_all(
        conn,
        "SELECT episode_id, name FROM episode_writers \
         WHERE episode_id IN (SELECT value FROM json_each(?1)) \
         ORDER BY episode_id, position",
        (format!("[{ids}]"),),
        |row| Ok((row.get::<i64>(0)?, row.get::<String>(1)?)),
    )
    .await?;

    let mut writers: HashMap<i64, Vec<String>> = HashMap::new();
    for (episode_id, name) in rows {
        writers.entry(episode_id).or_default().push(name);
    }
    for episode in episodes {
        episode.writers = writers.remove(&episode.id).unwrap_or_default();
    }

    Ok(())
}
