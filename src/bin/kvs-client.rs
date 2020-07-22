use std::process::exit;

use structopt::StructOpt;

use kvs::{ClientCommand, KvStore, KvsClientOpt, KvsEngine, KvsError, Result};

fn main() -> Result<()> {
    let opt = KvsClientOpt::from_args();
    match opt.cmd {
        ClientCommand::Get { key, .. } => {
            let mut kvs = KvStore::open(std::env::current_dir()?.as_path())?;
            if let Some(value) = kvs.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        ClientCommand::Set { key, value, .. } => {
            let mut kvs = KvStore::open(std::env::current_dir()?.as_path())?;
            kvs.set(key, value)?;
            Ok(())
        }
        ClientCommand::Remove { key, .. } => {
            let mut kvs = KvStore::open(std::env::current_dir()?.as_path())?;
            match kvs.remove(key) {
                Ok(()) => Ok(()),
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => Err(e),
            }
        }
    }
}
