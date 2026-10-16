//! Library crate for the API so integration tests in `tests/` can build the
//! router directly. `main.rs` is a thin binary around [`app::router`].

pub mod app;
pub mod config;
pub mod db;
pub mod error;
pub mod extract;
pub mod models;
pub mod openapi;
pub mod response;
pub mod routes;
pub mod state;
