pub mod connection;
pub mod db;
pub mod resp;
pub mod server;

/// Default port (real Redis uses 6379).
pub const DEFAULT_PORT: u16 = 6379;
