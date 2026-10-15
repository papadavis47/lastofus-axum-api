use crate::db::Db;

/// Shared application state. Cloning is cheap: `Db` is a handle, and each
/// request opens its own connection from it.
#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

impl AppState {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}
