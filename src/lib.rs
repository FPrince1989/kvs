#![deny(missing_docs)]
//! A key-value store

mod cli;
mod kv;

pub use kv::KvStore;
pub use cli::KvsOpt;
pub use cli::Command;
