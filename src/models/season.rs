use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Season {
    pub id: u32,
    pub number: u32,
    pub title: String,
    pub release_year: u16,
    pub episode_count: u32,
    pub summary: String,
}
