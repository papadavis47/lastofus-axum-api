use serde::Serialize;
use turso::Row;
use utoipa::ToSchema;

use super::{ExternalIds, Image, image::image_columns};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Episode {
    /// Equal to `overall_number`.
    #[schema(example = 1)]
    pub id: i64,
    pub season_id: i64,
    pub number_in_season: i64,
    pub overall_number: i64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    #[schema(required)]
    pub runtime_minutes: Option<i64>,
    /// ISO 8601 date, e.g. `2023-01-15`.
    #[schema(format = Date, example = "2023-01-15")]
    pub release_date: String,
    pub director: String,
    /// In credited order.
    // Loaded separately from `episode_writers`.
    pub writers: Vec<String>,
    /// `null` until reliable figures are available.
    #[schema(required)]
    pub viewer_count_millions: Option<f64>,
    #[schema(required)]
    pub image: Option<Image>,
    pub external_ids: ExternalIds,
}

impl Episode {
    /// Tables to select `COLUMNS` from: `episodes` as `e`, joined to its image.
    pub const FROM: &str = "episodes e LEFT JOIN images i ON i.id = e.image_id";

    /// Columns in the order `from_row` reads them.
    pub const COLUMNS: &str = concat!(
        "e.id, e.season_id, e.number_in_season, e.overall_number, \
         e.title, e.slug, e.summary, e.runtime_minutes, e.release_date, e.director, \
         e.viewer_count_millions, e.tmdb_id, e.imdb_id, e.wikidata_id, ",
        image_columns!()
    );

    /// Builds an episode with empty `writers`; callers fill them in.
    pub fn from_row(row: &Row) -> turso::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            season_id: row.get(1)?,
            number_in_season: row.get(2)?,
            overall_number: row.get(3)?,
            title: row.get(4)?,
            slug: row.get(5)?,
            summary: row.get(6)?,
            runtime_minutes: row.get(7)?,
            release_date: row.get(8)?,
            director: row.get(9)?,
            writers: Vec::new(),
            viewer_count_millions: row.get(10)?,
            external_ids: ExternalIds::from_row(row, 11)?,
            image: Image::from_row(row, 14)?,
        })
    }
}
