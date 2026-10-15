use std::{env, net::SocketAddr};

const DEFAULT_PORT: u16 = 3000;

pub struct Config {
    pub addr: SocketAddr,
}

impl Config {
    /// Reads configuration from the environment. `PORT` defaults to 3000.
    pub fn from_env() -> Result<Self, String> {
        let port = match env::var("PORT") {
            Ok(value) => value
                .parse()
                .map_err(|_| format!("PORT must be a valid port number, got {value:?}"))?,
            Err(_) => DEFAULT_PORT,
        };

        Ok(Self {
            addr: SocketAddr::from(([0, 0, 0, 0], port)),
        })
    }
}
