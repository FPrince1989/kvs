// #![deny(missing_docs)]
//! A key-value store

pub use cli::ClientCommand;
pub use cli::Engine;
pub use cli::KvsClientOpt;
pub use cli::KvsServerOpt;
pub use client::KvsClient;
pub use error::KvsError;
pub use error::Result;
pub use network::Request;
pub use network::Response;
pub use server::KvsServer;

pub use crate::kvs::KvStore;
pub use crate::kvs::SharedKvStore;
pub use crate::sled::SledKvsEngine;
use once_cell::sync::Lazy;
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::Severity;
use sloggers::Build;

mod cli;
mod client;
mod error;
mod kvs;
mod network;
mod server;
mod sled;
pub mod thread_pool;

/// defines the storage interface
pub trait KvsEngine: Clone + Send + 'static {
    /// set key and value
    fn set(&self, key: String, value: String) -> Result<()>;

    /// get value by key
    fn get(&self, key: String) -> Result<Option<String>>;

    /// remove by key
    fn remove(&self, key: String) -> Result<()>;

    /// get engine's Name
    fn name(&self) -> String;
}

#[derive(Debug)]
pub struct Logging {
    pub logger: slog::Logger,
}
pub static LOGGING: Lazy<Logging> = Lazy::new(|| {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);
    let logger = builder.build().unwrap();
    Logging { logger }
});
