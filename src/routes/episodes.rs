use std::collections::HashMap;

use axum::{Json, extract::State};
use serde::Deserialize;
use turso::Connection;
use utoipa::IntoParams;

use crate::{
    db,
    error::{AppError, ErrorResponse},
    extract::{ApiPath, ApiQuery},
    models::{Character, Episode},
    response::{ApiResponse, Paginated, Pagination},
    routes::characters::{self, CharacterQuery},
    state::AppState,
};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct EpisodeFilter {
    /// Only episodes from this season number. An unknown season gives an empty page.
    #[param(example = 1)]
    pub season: Option<u32>,
}

/// Narrows a page of episodes. Every field is optional.
#[derive(Debug, Default, Clone, Copy)]
pub struct EpisodeQuery {
    /// Only episodes from this season number.
    pub season: Option<u32>,
    /// Only episodes this character is credited in.
    pub character_id: Option<i64>,
}

/// List episodes
///
/// Every episode in broadcast order, one page at a time, optionally filtered by season.
#[utoipa::path(
    get,
    path = "/episodes",
    tag = "episodes",
    params(EpisodeFilter, Pagination),
    responses(
        (status = OK, description = "A page of episodes.", body = Paginated<Episode>),
        (status = BAD_REQUEST, description = "`invalid_pagination` or `invalid_query_parameter`.", body = ErrorResponse),
        (status = INTERNAL_SERVER_ERROR, description = "`internal_error`.", body = ErrorResponse),
    ),
)]
pub async fn list_episodes(
    State(state): State<AppState>,
    ApiQuery(filter): ApiQuery<EpisodeFilter>,
    ApiQuery(pagination): ApiQuery<Pagination>,
) -> Result<Json<Paginated<Episode>>, AppError> {
    let pagination = pagination.validate()?;
    let conn = state.db.connect().await?;

    // An unknown season matches nothing, so it yields an empty page, not a 404.
    let query = EpisodeQuery {
        season: filter.season,
        character_id: None,
    };
    let page = fetch_page(&conn, query, pagination).await?;

    Ok(Json(page))
}

/// Get an episode
///
/// One episode by id. Ids follow broadcast order, so they equal `overall_number`.
#[utoipa::path(
    get,
    path = "/episodes/{id}",
    tag = "episodes",
    params(("id" = u32, Path, description = "The episode id, e.g. `1`.", example = 1)),
    responses(
        (status = OK, description = "The episode.", body = ApiResponse<Episode>),
        (status = BAD_REQUEST, description = "`invalid_path_parameter`.", body = ErrorResponse),
        (status = NOT_FOUND, description = "`episode_not_found`.", body = ErrorResponse),
        (status = INTERNAL_SERVER_ERROR, description = "`internal_error`.", body = ErrorResponse),
    ),
)]
pub async fn get_episode(
    State(state): State<AppState>,
    ApiPath(id): ApiPath<u32>,
) -> Result<Json<ApiResponse<Episode>>, AppError> {
    let conn = state.db.connect().await?;

    let episode = find_episode(&conn, id).await?;
    let mut episodes = [episode];
    load_writers(&conn, &mut episodes).await?;
    let [episode] = episodes;

    Ok(Json(ApiResponse::new(episode)))
}

/// List an episode's characters
///
/// The characters credited in an episode, in alphabetical order, one page at a time.
#[utoipa::path(
    get,
    path = "/episodes/{id}/characters",
    tag = "episodes",
    params(
        ("id" = u32, Path, description = "The episode id, e.g. `1`.", example = 1),
        Pagination,
    ),
    responses(
        (status = OK, description = "A page of the episode's characters.", body = Paginated<Character>),
        (status = BAD_REQUEST, description = "`invalid_pagination`, `invalid_path_parameter` or `invalid_query_parameter`.", body = ErrorResponse),
        (status = NOT_FOUND, description = "`episode_not_found`.", body = ErrorResponse),
        (status = INTERNAL_SERVER_ERROR, description = "`internal_error`.", body = ErrorResponse),
    ),
)]
pub async fn list_episode_characters(
    State(state): State<AppState>,
    ApiPath(id): ApiPath<u32>,
    ApiQuery(pagination): ApiQuery<Pagination>,
) -> Result<Json<Paginated<Character>>, AppError> {
    let pagination = pagination.validate()?;
    let conn = state.db.connect().await?;

    find_episode(&conn, id).await?;
    let query = CharacterQuery {
        episode_id: Some(id),
        ..CharacterQuery::default()
    };
    let page = characters::fetch_page(&conn, query, pagination).await?;

    Ok(Json(page))
}

/// One page of episodes in broadcast order. Shared with
/// `GET /seasons/{season_number}/episodes` and `GET /characters/{slug}/episodes`.
pub async fn fetch_page(
    conn: &Connection,
    query: EpisodeQuery,
    pagination: Pagination,
) -> Result<Paginated<Episode>, AppError> {
    let filter = format!(
        "FROM {} JOIN seasons s ON s.id = e.season_id \
         WHERE (?1 IS NULL OR s.number = ?1) \
         AND (?2 IS NULL OR e.id IN (SELECT episode_id FROM episode_characters WHERE character_id = ?2))",
        Episode::FROM
    );

    let total = db::count(
        conn,
        &format!("SELECT COUNT(*) {filter}"),
        (query.season, query.character_id),
    )
    .await?;
    let mut episodes = db::query_all(
        conn,
        &format!(
            "SELECT {} {filter} ORDER BY e.overall_number LIMIT ?3 OFFSET ?4",
            Episode::COLUMNS
        ),
        (
            query.season,
            query.character_id,
            pagination.limit,
            pagination.offset(),
        ),
        Episode::from_row,
    )
    .await?;
    load_writers(conn, &mut episodes).await?;

    Ok(pagination.page(episodes, total))
}

async fn find_episode(conn: &Connection, id: u32) -> Result<Episode, AppError> {
    db::query_optional(
        conn,
        &format!(
            "SELECT {} FROM {} WHERE e.id = ?1",
            Episode::COLUMNS,
            Episode::FROM
        ),
        (id,),
        Episode::from_row,
    )
    .await?
    .ok_or_else(|| AppError::not_found("episode_not_found", format!("Episode {id} was not found.")))
}

/// Fills in `writers` for every episode with a single query.
async fn load_writers(conn: &Connection, episodes: &mut [Episode]) -> Result<(), AppError> {
    if episodes.is_empty() {
        return Ok(());
    }

    let rows = db::query_all(
        conn,
        "SELECT episode_id, name FROM episode_writers \
         WHERE episode_id IN (SELECT value FROM json_each(?1)) \
         ORDER BY episode_id, position",
        (db::json_ids(episodes.iter().map(|episode| episode.id)),),
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
