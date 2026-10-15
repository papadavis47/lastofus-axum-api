use serde::Serialize;
use turso::Row;

#[derive(Debug, Clone, Serialize)]
pub struct Episode {
    pub id: i64,
    pub season_id: i64,
    pub number_in_season: i64,
    pub overall_number: i64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub runtime_minutes: Option<i64>,
    /// ISO 8601 date, e.g. `2023-01-15`.
    pub release_date: String,
    pub director: String,
    /// In credited order. Loaded separately from `episode_writers`.
    pub writers: Vec<String>,
    pub viewer_count_millions: Option<f64>,
    pub image_url: Option<String>,
}

impl Episode {
    /// Columns (on the `episodes` table aliased as `e`) in the order
    /// `from_row` reads them.
    pub const COLUMNS: &str = "e.id, e.season_id, e.number_in_season, e.overall_number, \
         e.title, e.slug, e.summary, e.runtime_minutes, e.release_date, e.director, \
         e.viewer_count_millions, e.image_url";

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
            image_url: row.get(11)?,
        })
    }
}
