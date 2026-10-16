use serde::Serialize;
use turso::Row;
use utoipa::ToSchema;

use super::{ExternalIds, Image, image::image_columns};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Season {
    pub id: i64,
    #[schema(example = 1)]
    pub number: i64,
    pub title: String,
    #[schema(example = 2023)]
    pub release_year: i64,
    pub episode_count: i64,
    pub summary: String,
    #[schema(required)]
    pub image: Option<Image>,
    pub external_ids: ExternalIds,
}

impl Season {
    /// Tables to select `COLUMNS` from: `seasons` as `s`, joined to its image.
    pub const FROM: &str = "seasons s LEFT JOIN images i ON i.id = s.image_id";

    /// Columns in the order `from_row` reads them.
    pub const COLUMNS: &str = concat!(
        "s.id, s.number, s.title, s.release_year, s.episode_count, s.summary, \
         s.tmdb_id, s.imdb_id, s.wikidata_id, ",
        image_columns!()
    );

    pub fn from_row(row: &Row) -> turso::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            number: row.get(1)?,
            title: row.get(2)?,
            release_year: row.get(3)?,
            episode_count: row.get(4)?,
            summary: row.get(5)?,
            external_ids: ExternalIds::from_row(row, 6)?,
            image: Image::from_row(row, 9)?,
        })
    }
}
