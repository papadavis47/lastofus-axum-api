use lastofus_axum_api::{
    app,
    config::Config,
    db::{Db, migrations},
    state::AppState,
};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            // The sync engine logs every pull at INFO; keep it to warnings by default.
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,turso_sync_engine=warn")),
        )
        .init();

    let config = Config::from_env()?;
    let db = Db::open(&config.database).await?;
    prepare_schema(&db).await?;

    let listener = TcpListener::bind(config.addr).await?;
    info!(address = %listener.local_addr()?, "server listening");

    axum::serve(listener, app::router(AppState::new(db)))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Local databases are migrated on startup so `cargo run` just works. A synced
/// replica is read-only for the server: migrations reach Turso Cloud through
/// `cargo run --bin migrate`, so here we only refuse to start if any are missing.
async fn prepare_schema(db: &Db) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db.connect().await?;

    if db.is_synced() {
        let pending = migrations::pending(&conn).await?;
        if !pending.is_empty() {
            return Err(format!(
                "Turso Cloud database is missing migrations {pending:?}; \
                 run `cargo run --bin migrate` with the same TURSO_* settings"
            )
            .into());
        }
        info!("synced database is up to date");
    } else {
        let applied = migrations::run(&conn).await?;
        info!(?applied, "local database migrated");
    }

    Ok(())
}

/// Resolves on Ctrl+C (or SIGTERM on Unix) so in-flight requests can finish.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    info!("shutdown signal received");
}
