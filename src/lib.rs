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