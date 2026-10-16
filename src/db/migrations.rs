//! A minimal migration runner.
//!
//! Migrations are the numbered `.sql` files in `migrations/`, embedded at
//! compile time. Applied versions are recorded in a `_migrations` table rather
//! than `PRAGMA user_version`, which is read-only on Turso Cloud.

use turso::Connection;

pub struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub sql: &'static str,
}

/// Every migration, in order. Add new files here; never edit applied ones.
pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "create_seasons_and_episodes",
        sql: include_str!("../../migrations/0001_create_seasons_and_episodes.sql"),
    },
    Migration {
        version: 2,
        name: "seed_seasons_1_and_2",
        sql: include_str!("../../migrations/0002_seed_seasons_1_and_2.sql"),
    },
    Migration {
        version: 3,
        name: "images_and_external_ids",
        sql: include_str!("../../migrations/0003_images_and_external_ids.sql"),
    },
];

/// Applies every pending migration, each in its own transaction, and returns
/// the versions that were applied.
pub async fn run(conn: &Connection) -> turso::Result<Vec<i64>> {
    // Off by default in SQLite and Turso; migrations are where writes happen.
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;
    ensure_table(conn).await?;
    let current = current_version(conn).await?;
    let mut applied = Vec::new();

    for migration in MIGRATIONS.iter().filter(|m| m.version > current) {
        let batch = format!(
            "BEGIN;\n{}\nINSERT INTO _migrations (version, name) VALUES ({}, '{}');\nCOMMIT;",
            migration.sql, migration.version, migration.name
        );
        if let Err(err) = conn.execute_batch(&batch).await {
            // Best effort: the transaction may never have started.
            let _ = conn.execute("ROLLBACK", ()).await;
            return Err(err);
        }
        applied.push(migration.version);
    }

    Ok(applied)
}

/// Versions defined in code but not yet applied to this database. Read-only:
/// safe to call against a synced replica.
pub async fn pending(conn: &Connection) -> turso::Result<Vec<i64>> {
    let has_table = super::count(
        conn,
        "SELECT COUNT(*) FROM sqlite_schema WHERE type = 'table' AND name = '_migrations'",
        (),
    )
    .await?
        > 0;
    let current = if has_table {
        current_version(conn).await?
    } else {
        0
    };
    Ok(MIGRATIONS
        .iter()
        .map(|m| m.version)
        .filter(|&version| version > current)
        .collect())
}

async fn ensure_table(conn: &Connection) -> turso::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _migrations (
            version    INTEGER PRIMARY KEY,
            name       TEXT NOT NULL,
            applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )
    .await?;
    Ok(())
}

async fn current_version(conn: &Connection) -> turso::Result<i64> {
    let version = super::query_optional(conn, "SELECT MAX(version) FROM _migrations", (), |row| {
        row.get::<Option<i64>>(0)
    })
    .await?;
    Ok(version.flatten().unwrap_or(0))
}
