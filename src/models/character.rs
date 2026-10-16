use serde::{Deserialize, Serialize};
use turso::Row;
use utoipa::ToSchema;

use super::{Image, image::image_columns};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Character {
    #[schema(example = 1)]
    pub id: i64,
    /// URL-safe name used to look the character up, e.g. `joel-miller`.
    #[schema(example = "joel-miller")]
    pub slug: String,
    #[schema(example = "Joel Miller")]
    pub name: String,
    /// A short, original description, light on spoilers.
    pub description: String,
    /// Every season the character appears in, in order.
    // Loaded separately from `character_seasons`.
    pub seasons: Vec<CharacterSeason>,
    #[schema(required)]
    pub image: Option<Image>,
}

/// A character's part in one season.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CharacterSeason {
    /// The season number.
    #[schema(example = 1)]
    pub season: i64,
    pub role: Role,
    /// Episodes of this season the character is credited in.
    #[schema(example = 9)]
    pub episode_count: i64,
}

/// How a character is billed in a season.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Main,
    Recurring,
    Guest,
}

impl Role {
    /// The value stored in `character_seasons.role`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Recurring => "recurring",
            Self::Guest => "guest",
        }
    }

    pub fn parse(value: &str) -> turso::Result<Self> {
        match value {
            "main" => Ok(Self::Main),
            "recurring" => Ok(Self::Recurring),
            "guest" => Ok(Self::Guest),
            other => Err(turso::Error::ConversionFailure(format!(
                "unknown character role {other:?}"
            ))),
        }
    }
}

impl Character {
    /// Tables to select `COLUMNS` from: `characters` as `c`, joined to its image.
    pub const FROM: &str = "characters c LEFT JOIN images i ON i.id = c.image_id";

    /// Columns in the order `from_row` reads them.
    pub const COLUMNS: &str = concat!("c.id, c.slug, c.name, c.description, ", image_columns!());

    /// Builds a character with empty `seasons`; callers fill them in.
    pub fn from_row(row: &Row) -> turso::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            slug: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            seasons: Vec::new(),
            image: Image::from_row(row, 4)?,
        })
    }
}
