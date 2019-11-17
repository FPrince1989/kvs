use structopt::StructOpt;

use kvs::{Command, KvStore, KvsError};
use kvs::KvsOpt;
use kvs::Result;
use std::process::exit;

fn main() -> Result<()> {
    let opt = KvsOpt::from_args();
    match opt.cmd {
        Command::Get { key } => {
            let mut kvs = KvStore::open(std::env::current_dir()?.as_path())?;
            if let Some(value) = kvs.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        Command::Set { key, value } => {
            let mut kvs = KvStore::open(std::env::current_dir()?.as_path())?;
            kvs.set(key, value)?;
            Ok(())
        }
        Command::Remove { key } => {
            let mut kvs = KvStore::open(std::env::current_dir()?.as_path())?;
            match kvs.remove(key) {
                Ok(()) => {
                    Ok(())
                },
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                },
                Err(e) => {
                    Err(e)
                }
            }
        }
    }

    // let matches = App::new(env!("CARGO_PKG_NAME"))
    //     .version(env!("CARGO_PKG_VERSION"))
    //     .author(env!("CARGO_PKG_AUTHORS"))
    //     .about(env!("CARGO_PKG_DESCRIPTION"))
    //     .subcommand(
    //         SubCommand::with_name("get")
    //             .about("Get the string value of a given string key")
    //             .arg(Arg::with_name("KEY").required(true)),
    //     )
    //     .subcommand(
    //         SubCommand::with_name("set")
    //             .about("Set the value of a string key to a string")
    //             .arg(Arg::with_name("KEY").required(true))
    //             .arg(Arg::with_name("VALUE").required(true)),
    //     )
    //     .subcommand(
    //         SubCommand::with_name("rm")
    //             .about("Remove a given key")
    //             .arg(Arg::with_name("KEY").required(true)),
    //     )
    //     .get_matches();
    // if let Some(get_matches) = matches.subcommand_matches("get") {
    //     let key = get_matches.value_of("KEY");
    //     println!("{}", key.unwrap());
    //     eprintln!("unimplemented");
    //     std::process::exit(1);
    // } else if let Some(set_matches) = matches.subcommand_matches("set") {
    //     let key = set_matches.value_of("KEY");
    //     let value = set_matches.value_of("VALUE");
    //     println!("{} {}", key.unwrap(), value.unwrap());
    //     eprintln!("unimplemented");
    //     std::process::exit(1);
    // } else if let Some(rm_matches) = matches.subcommand_matches("rm") {
    //     let key = rm_matches.value_of("KEY");
    //     println!("{}", key.unwrap());
    //     eprintln!("unimplemented");
    //     std::process::exit(1);
    // } else {
    //     std::process::exit(1);
    // }
}
