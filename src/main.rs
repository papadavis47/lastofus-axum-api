mod app;
mod error;
mod routes;

use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!(address = %listener.local_addr()?, "server listening");

    axum::serve(listener, app::router()).await
}
