use std::collections::HashMap;

use axum::{Json, extract::State};
use serde::Deserialize;
use turso::Connection;
use utoipa::IntoParams;

use crate::{
    db,
    error::{AppError, ErrorResponse},
    extract::{ApiPath, ApiQuery},
    models::{Character, CharacterSeason, Episode, Role},
    response::{ApiResponse, Paginated, Pagination},
    routes::episodes::{self, EpisodeFilter, EpisodeQuery},
    state::AppState,
};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct CharacterFilter {
    /// Only characters who appear in this season number. An unknown season gives an empty page.
    #[param(example = 2)]
    season: Option<u32>,
    /// Only characters with this billing, in any season or in `season` if it's given.
    #[param(inline)]
    role: Option<Role>,
}

/// Narrows a page of characters. Every field is optional.
#[derive(Debug, Default, Clone, Copy)]
pub struct CharacterQuery {
    pub season: Option<u32>,
    pub role: Option<Role>,
    /// Only characters credited in this episode.
    pub episode_id: Option<u32>,
}

/// List characters
///
/// Characters in alphabetical order, one page at a time, optionally filtered by season and billing.
#[utoipa::path(
    get,
    path = "/characters",
    tag = "characters",
    params(CharacterFilter, Pagination),
    responses(
        (status = OK, description = "A page of characters.", body = Paginated<Character>),
        (status = BAD_REQUEST, description = "`invalid_pagination` or `invalid_query_parameter`.", body = ErrorResponse),
        (status = INTERNAL_SERVER_ERROR, description = "`internal_error`.", body = ErrorResponse),
    ),
)]
pub async fn list_characters(
    State(state): State<AppState>,
    ApiQuery(filter): ApiQuery<CharacterFilter>,
    ApiQuery(pagination): ApiQuery<Pagination>,
) -> Result<Json<Paginated<Character>>, AppError> {
    let pagination = pagination.validate()?;
    let conn = state.db.connect().await?;

    let query = CharacterQuery {
        season: filter.season,
        role: filter.role,
        episode_id: None,
    };
    let page = fetch_page(&conn, query, pagination).await?;

    Ok(Json(page))
}

/// Get a character
///
/// One character by slug, with the seasons they appear in.
#[utoipa::path(
    get,
    path = "/characters/{slug}",
    tag = "characters",
    params(("slug" = String, Path, description = "The character's slug, e.g. `joel-miller`.", example = "joel-miller")),
    responses(
        (status = OK, description = "The character.", body = ApiResponse<Character>),
        (status = NOT_FOUND, description = "`character_not_found`.", body = ErrorResponse),
        (status = INTERNAL_SERVER_ERROR, description = "`internal_error`.", body = ErrorResponse),
    ),
)]
pub async fn get_character(
    State(state): State<AppState>,
    ApiPath(slug): ApiPath<String>,
) -> Result<Json<ApiResponse<Character>>, AppError> {
    let conn = state.db.connect().await?;

    let character = find_character(&conn, &slug).await?;
    let mut characters = [character];
    load_seasons(&conn, &mut characters).await?;
    let [character] = characters;

    Ok(Json(ApiResponse::new(character)))
}

/// List a character's episodes
///
/// The episodes a character is credited in, in broadcast order, one page at a time.
#[utoipa::path(
    get,
    path = "/characters/{slug}/episodes",
    tag = "characters",
    params(
        ("slug" = String, Path, description = "The character's slug, e.g. `joel-miller`.", example = "joel-miller"),
        EpisodeFilter,
        Pagination,
    ),
    responses(
        (status = OK, description = "A page of the character's episodes.", body = Paginated<Episode>),
        (status = BAD_REQUEST, description = "`invalid_pagination` or `invalid_query_parameter`.", body = ErrorResponse),
        (status = NOT_FOUND, description = "`character_not_found`.", body = ErrorResponse),
        (status = INTERNAL_SERVER_ERROR, description = "`internal_error`.", body = ErrorResponse),
    ),
)]
pub async fn list_character_episodes(
    State(state): State<AppState>,
    ApiPath(slug): ApiPath<String>,
    ApiQuery(filter): ApiQuery<EpisodeFilter>,
    ApiQuery(pagination): ApiQuery<Pagination>,
) -> Result<Json<Paginated<Episode>>, AppError> {
    let pagination = pagination.validate()?;
    let conn = state.db.connect().await?;

    let character = find_character(&conn, &slug).await?;
    let query = EpisodeQuery {
        season: filter.season,
        character_id: Some(character.id),
    };
    let page = episodes::fetch_page(&conn, query, pagination).await?;

    Ok(Json(page))
}

/// One page of characters in alphabetical order. Shared with
/// `GET /episodes/{id}/characters`.
pub async fn fetch_page(
    conn: &Connection,
    query: CharacterQuery,
    pagination: Pagination,
) -> Result<Paginated<Character>, AppError> {
    // Every character has at least one season, so the EXISTS only filters when
    // `season` or `role` is set.
    let filter = format!(
        "FROM {} \
         WHERE EXISTS ( \
             SELECT 1 FROM character_seasons cs JOIN seasons s ON s.id = cs.season_id \
             WHERE cs.character_id = c.id \
               AND (?1 IS NULL OR s.number = ?1) \
               AND (?2 IS NULL OR cs.role = ?2)) \
         AND (?3 IS NULL OR c.id IN (SELECT character_id FROM episode_characters WHERE episode_id = ?3))",
        Character::FROM
    );
    let role = query.role.map(Role::as_str);

    let total = db::count(
        conn,
        &format!("SELECT COUNT(*) {filter}"),
        (query.season, role, query.episode_id),
    )
    .await?;
    let mut characters = db::query_all(
        conn,
        &format!(
            "SELECT {} {filter} ORDER BY c.name, c.id LIMIT ?4 OFFSET ?5",
            Character::COLUMNS
        ),
        (
            query.season,
            role,
            query.episode_id,
            pagination.limit,
            pagination.offset(),
        ),
        Character::from_row,
    )
    .await?;
    load_seasons(conn, &mut characters).await?;

    Ok(pagination.page(characters, total))
}

async fn find_character(conn: &Connection, slug: &str) -> Result<Character, AppError> {
    db::query_optional(
        conn,
        &format!(
            "SELECT {} FROM {} WHERE c.slug = ?1",
            Character::COLUMNS,
            Character::FROM
        ),
        (slug,),
        Character::from_row,
    )
    .await?
    .ok_or_else(|| {
        AppError::not_found(
            "character_not_found",
            format!("Character \"{slug}\" was not found."),
        )
    })
}

/// Fills in `seasons` for every character with a single query.
async fn load_seasons(conn: &Connection, characters: &mut [Character]) -> Result<(), AppError> {
    if characters.is_empty() {
        return Ok(());
    }

    let rows = db::query_all(
        conn,
        "SELECT cs.character_id, s.number, cs.role, \
             (SELECT COUNT(*) FROM episode_characters ec JOIN episodes e ON e.id = ec.episode_id \
              WHERE ec.character_id = cs.character_id AND e.season_id = cs.season_id) \
         FROM character_seasons cs JOIN seasons s ON s.id = cs.season_id \
         WHERE cs.character_id IN (SELECT value FROM json_each(?1)) \
         ORDER BY cs.character_id, s.number",
        (db::json_ids(
            characters.iter().map(|character| character.id),
        ),),
        |row| {
            let season = CharacterSeason {
                season: row.get(1)?,
                role: Role::parse(&row.get::<String>(2)?)?,
                episode_count: row.get(3)?,
            };
            Ok((row.get::<i64>(0)?, season))
        },
    )
    .await?;

    let mut seasons: HashMap<i64, Vec<CharacterSeason>> = HashMap::new();
    for (character_id, season) in rows {
        seasons.entry(character_id).or_default().push(season);
    }
    for character in characters {
        character.seasons = seasons.remove(&character.id).unwrap_or_default();
    }

    Ok(())
}
