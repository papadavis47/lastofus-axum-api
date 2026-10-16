use serde::Serialize;
use turso::Row;
use utoipa::ToSchema;

/// Columns on the `images` table aliased as `i`, in the order `Image::from_row`
/// reads them. A macro rather than a `const` so models can `concat!` it onto
/// their own `COLUMNS`. Queries `LEFT JOIN` the table, so every column may be NULL.
macro_rules! image_columns {
    () => {
        "i.url, i.width, i.height, i.alt_text, i.attribution_text, \
         i.creator, i.license_name, i.license_url, i.source_page_url"
    };
}
pub(crate) use image_columns;

/// A licensed image plus the attribution its licence requires.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Image {
    pub url: String,
    #[schema(required)]
    pub width: Option<i64>,
    #[schema(required)]
    pub height: Option<i64>,
    pub alt: String,
    pub attribution: Attribution,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Attribution {
    /// Ready-to-display credit line.
    pub text: String,
    #[schema(required)]
    pub creator: Option<String>,
    pub license: String,
    #[schema(required)]
    pub license_url: Option<String>,
    /// The page the licence was checked on.
    pub source_url: String,
}

impl Image {
    /// Reads `image_columns!()` starting at column `start`. Returns `None` when the
    /// join found no image.
    pub fn from_row(row: &Row, start: usize) -> turso::Result<Option<Self>> {
        let Some(url) = row.get::<Option<String>>(start)? else {
            return Ok(None);
        };
        Ok(Some(Self {
            url,
            width: row.get(start + 1)?,
            height: row.get(start + 2)?,
            alt: row.get(start + 3)?,
            attribution: Attribution {
                text: row.get(start + 4)?,
                creator: row.get(start + 5)?,
                license: row.get(start + 6)?,
                license_url: row.get(start + 7)?,
                source_url: row.get(start + 8)?,
            },
        }))
    }
}

/// Ids for the same resource in other databases. API users can use them to
/// fetch artwork from those services under their own terms.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ExternalIds {
    #[schema(required)]
    pub tmdb: Option<i64>,
    #[schema(required, example = "tt11957006")]
    pub imdb: Option<String>,
    #[schema(required, example = "Q116169494")]
    pub wikidata: Option<String>,
}

impl ExternalIds {
    /// Reads `tmdb_id, imdb_id, wikidata_id` starting at column `start`.
    pub fn from_row(row: &Row, start: usize) -> turso::Result<Self> {
        Ok(Self {
            tmdb: row.get(start)?,
            imdb: row.get(start + 1)?,
            wikidata: row.get(start + 2)?,
        })
    }
}
