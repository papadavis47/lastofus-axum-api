use std::{env, net::SocketAddr};

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_DATABASE_PATH: &str = "lastofus.db";

pub struct Config {
    pub addr: SocketAddr,
    pub database: DatabaseConfig,
}

pub struct DatabaseConfig {
    /// Local database file. In synced mode this is the local replica.
    pub path: String,
    /// Set when `TURSO_DATABASE_URL` is present: sync with Turso Cloud.
    pub remote: Option<RemoteConfig>,
}

pub struct RemoteConfig {
    pub url: String,
    /// Optional so a local `tursodb --sync-server` works without a token.
    pub auth_token: Option<String>,
}

impl Config {
    /// Reads configuration from the environment:
    ///
    /// - `PORT` (default 3000)
    /// - `DATABASE_PATH` (default `lastofus.db`)
    /// - `TURSO_DATABASE_URL` and `TURSO_AUTH_TOKEN` (optional; enable sync)
    pub fn from_env() -> Result<Self, String> {
        let port = match env::var("PORT") {
            Ok(value) => value
                .parse()
                .map_err(|_| format!("PORT must be a valid port number, got {value:?}"))?,
            Err(_) => DEFAULT_PORT,
        };

        Ok(Self {
            addr: SocketAddr::from(([0, 0, 0, 0], port)),
            database: DatabaseConfig::from_env(),
        })
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let remote = non_empty_var("TURSO_DATABASE_URL").map(|url| RemoteConfig {
            url,
            auth_token: non_empty_var("TURSO_AUTH_TOKEN"),
        });

        Self {
            path: non_empty_var("DATABASE_PATH").unwrap_or_else(|| DEFAULT_DATABASE_PATH.into()),
            remote,
        }
    }
}

fn non_empty_var(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.is_empty())
}
