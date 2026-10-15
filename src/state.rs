use std::sync::Arc;

use crate::{
    fixtures,
    models::{Episode, Season},
};

/// Shared, read-only application state. Cloning is cheap because the data
/// lives behind an `Arc`. In Phase 3 this will hold an `SqlitePool` instead.
#[derive(Clone)]
pub struct AppState {
    pub seasons: Arc<[Season]>,
    pub episodes: Arc<[Episode]>,
}

impl AppState {
    pub fn from_fixtures() -> Self {
        Self {
            seasons: fixtures::seasons().into(),
            episodes: fixtures::episodes().into(),
        }
    }
}
