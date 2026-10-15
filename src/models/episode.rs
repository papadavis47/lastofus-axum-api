use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Episode {
    pub id: u32,
    pub season_id: u32,
    pub number_in_season: u32,
    pub overall_number: u32,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub runtime_minutes: Option<u32>,
    /// ISO 8601 date, e.g. `2023-01-15`.
    pub release_date: String,
    pub director: String,
    pub writers: Vec<String>,
    pub viewer_count_millions: Option<f32>,
    pub image_url: Option<String>,
}
