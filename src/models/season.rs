use serde::Serialize;
use turso::Row;

#[derive(Debug, Clone, Serialize)]
pub struct Season {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub release_year: i64,
    pub episode_count: i64,
    pub summary: String,
}

impl Season {
    /// Columns in the order `from_row` reads them.
    pub const COLUMNS: &str = "id, number, title, release_year, episode_count, summary";

    pub fn from_row(row: &Row) -> turso::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            number: row.get(1)?,
            title: row.get(2)?,
            release_year: row.get(3)?,
            episode_count: row.get(4)?,
            summary: row.get(5)?,
        })
    }
}
