// #![deny(missing_docs)]
//! A key-value store

pub use crate::sled::SledKvsEngine;
pub use cli::ClientCommand;
pub use cli::Engine;
pub use cli::KvsClientOpt;
pub use cli::KvsServerOpt;
pub use client::KvsClient;
pub use error::KvsError;
pub use error::Result;
pub use kvs::KvStore;
pub use network::Request;
pub use network::Response;
pub use server::KvsServer;

mod cli;
mod client;
mod error;
mod kvs;
mod network;
mod server;
mod sled;

/// defines the storage interface
pub trait KvsEngine {
    /// set key and value
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// get value by key
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// remove by key
    fn remove(&mut self, key: String) -> Result<()>;

    /// get Engine's Name
    fn name(&self) -> String;
}
