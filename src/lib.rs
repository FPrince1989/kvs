#![deny(missing_docs)]
//! A key-value store

mod cli;
mod error;
mod kv;

pub use cli::ClientCommand;
pub use cli::KvsClientOpt;
pub use cli::KvsServerOpt;
pub use error::KvsError;
pub use error::Result;
pub use kv::KvStore;

/// defines the storage interface
pub trait KvsEngine {
    /// set key and value
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// get value by key
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// remove by key
    fn remove(&mut self, key: String) -> Result<()>;
}
