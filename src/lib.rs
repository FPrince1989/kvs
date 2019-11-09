#![deny(missing_docs)]
//! A key-value store

mod cli;
mod kv;

pub use cli::Command;
pub use cli::KvsOpt;
pub use kv::KvStore;
