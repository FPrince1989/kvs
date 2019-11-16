#![deny(missing_docs)]
//! A key-value store

mod cli;
mod kv;
mod error;

pub use cli::Command;
pub use cli::KvsOpt;
pub use kv::KvStore;
pub use error::KvsError;
pub use error::Result;