#![deny(missing_docs)]
//! A key-value store

mod cli;
mod error;
mod kv;

pub use cli::Command;
pub use cli::KvsOpt;
pub use error::KvsError;
pub use error::Result;
pub use kv::KvStore;
