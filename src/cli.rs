use std::net::SocketAddr;

use structopt::StructOpt;

/// KvsClientOpt
#[derive(Debug, StructOpt)]
#[structopt(name="kvs-client", author, about="A key-value store client")]
pub struct KvsClientOpt {
    /// the sub command
    #[structopt(subcommand)]
    pub cmd: ClientCommand,
}

/// Cli SubCommand
#[derive(Debug, StructOpt)]
pub enum ClientCommand {
    /// Get the string value of a given string key
    Get {
        /// the string key
        #[structopt(name = "KEY")]
        key: String,

        /// the remote address
        #[structopt(
            long,
            name = "IP-PORT",
            default_value = "127.0.0.1:4000",
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    /// Set the value of a string key to a string
    Set {
        /// the string key
        #[structopt(name = "KEY")]
        key: String,

        /// the string value
        #[structopt(name = "VALUE")]
        value: String,

        /// the remote address
        #[structopt(
            long,
            name = "IP-PORT",
            default_value = "127.0.0.1:4000",
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    /// Remove a given key
    #[structopt(name = "rm")]
    Remove {
        /// the string key
        #[structopt(name = "KEY")]
        key: String,

        /// the remote address
        #[structopt(
            long,
            name = "IP-PORT",
            default_value = "127.0.0.1:4000",
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
}

/// KvsServerOpt
#[derive(Debug, StructOpt)]
#[structopt(name="kvs-server", author, about="A key-value store server")]
pub struct KvsServerOpt {
    /// listening address
    #[structopt(
        long,
        name = "IP-PORT",
        default_value = "127.0.0.1:4000",
        parse(try_from_str)
    )]
    pub addr: SocketAddr,

    /// the storage engine
    #[structopt(long, name = "ENGINE-NAME", default_value = "kvs")]
    pub engine: String,
}
