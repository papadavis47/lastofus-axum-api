//! Applies pending migrations to the configured database.
//!
//! Locally this updates `DATABASE_PATH`. With `TURSO_DATABASE_URL` set it
//! updates the local replica and then pushes the changes to Turso Cloud, which
//! is how schema and data changes reach production.

use lastofus_axum_api::{
    config::DatabaseConfig,
    db::{Db, migrations},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = DatabaseConfig::from_env();
    let db = Db::open(&config).await?;
    let conn = db.connect().await?;

    let applied = migrations::run(&conn).await?;
    if applied.is_empty() {
        println!("Database is up to date.");
    } else {
        println!("Applied migrations: {applied:?}");
    }

    if db.is_synced() {
        db.push().await?;
        println!(
            "Pushed changes to {}.",
            config.remote.as_ref().map_or("", |r| r.url.as_str())
        );
    }

    Ok(())
}
