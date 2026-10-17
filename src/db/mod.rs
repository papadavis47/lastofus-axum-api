//! Turso database access: opening the database, connections, migrations,
//! and small query helpers shared by the route handlers.

pub mod migrations;

use turso::{Connection, Row, params::IntoParams};

use crate::config::DatabaseConfig;

/// A handle to the database, cheap to clone into `AppState`.
///
/// - `Local`: an embedded database file (or `:memory:` in tests).
/// - `Synced`: a local replica of a Turso Cloud database. Reads are served
///   locally; `pull` fetches remote changes and `push` sends local ones.
#[derive(Clone)]
pub enum Db {
    Local(turso::Database),
    Synced(turso::sync::Database),
}

impl Db {
    pub async fn open(config: &DatabaseConfig) -> turso::Result<Self> {
        match &config.remote {
            None => Ok(Self::Local(
                turso::Builder::new_local(&config.path).build().await?,
            )),
            Some(remote) => {
                let mut builder =
                    turso::sync::Builder::new_remote(&config.path).with_remote_url(&remote.url);
                if let Some(token) = &remote.auth_token {
                    builder = builder.with_auth_token(token);
                }
                // `build` downloads the database if the local file is empty;
                // `pull` brings an existing replica up to date.
                let db = builder.build().await?;
                db.pull().await?;
                Ok(Self::Synced(db))
            }
        }
    }

    /// A fresh, empty in-memory database. Used by tests.
    pub async fn in_memory() -> turso::Result<Self> {
        Ok(Self::Local(
            turso::Builder::new_local(":memory:").build().await?,
        ))
    }

    pub async fn connect(&self) -> turso::Result<Connection> {
        match self {
            Self::Local(db) => db.connect(),
            Self::Synced(db) => db.connect().await,
        }
    }

    pub fn is_synced(&self) -> bool {
        matches!(self, Self::Synced(_))
    }

    /// Sends local changes to Turso Cloud. A no-op for local databases.
    pub async fn push(&self) -> turso::Result<()> {
        match self {
            Self::Local(_) => Ok(()),
            Self::Synced(db) => db.push().await,
        }
    }
}

/// Runs a query and maps every row with `from_row`.
pub async fn query_all<T>(
    conn: &Connection,
    sql: &str,
    params: impl IntoParams,
    from_row: impl Fn(&Row) -> turso::Result<T>,
) -> turso::Result<Vec<T>> {
    let mut rows = conn.query(sql, params).await?;
    let mut items = Vec::new();
    while let Some(row) = rows.next().await? {
        items.push(from_row(&row)?);
    }
    Ok(items)
}

/// Runs a query and maps the first row, if any.
pub async fn query_optional<T>(
    conn: &Connection,
    sql: &str,
    params: impl IntoParams,
    from_row: impl Fn(&Row) -> turso::Result<T>,
) -> turso::Result<Option<T>> {
    let mut rows = conn.query(sql, params).await?;
    match rows.next().await? {
        Some(row) => Ok(Some(from_row(&row)?)),
        None => Ok(None),
    }
}

/// Runs a `SELECT COUNT(*) ...` query.
pub async fn count(conn: &Connection, sql: &str, params: impl IntoParams) -> turso::Result<u64> {
    let total = query_optional(conn, sql, params, |row| row.get::<i64>(0)).await?;
    Ok(total.unwrap_or(0).try_into().unwrap_or(0))
}

/// Formats ids as a JSON array, e.g. `[1,2,3]`, to bind as one parameter and
/// expand with `IN (SELECT value FROM json_each(?1))` instead of building
/// `IN (?, ?, ...)`.
pub fn json_ids(ids: impl IntoIterator<Item = i64>) -> String {
    let ids: Vec<String> = ids.into_iter().map(|id| id.to_string()).collect();
    format!("[{}]", ids.join(","))
}
