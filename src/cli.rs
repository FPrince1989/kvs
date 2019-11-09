use structopt::StructOpt;

/// KvsOpt
#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct KvsOpt {
    /// subcommand
    #[structopt(subcommand)]
    pub cmd: Command,
}

/// subcommand
#[derive(Debug, StructOpt)]
pub enum Command {
    /// Get the string value of a given string key
    Get {
        /// the string key
        key: String,
    },
    /// Set the value of a string key to a string
    Set {
        /// the string key
        key: String,
        /// the string value
        value: String,
    },
    /// Remove a given key
    #[structopt(name = "rm")]
    Remove {
        /// the string key
        key: String,
    },
}
