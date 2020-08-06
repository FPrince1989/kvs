use structopt::StructOpt;

use kvs::{ClientCommand, KvsClient, KvsClientOpt, Result};

fn main() -> Result<()> {
    let opt = KvsClientOpt::from_args();
    match opt.cmd {
        ClientCommand::Get { key, addr } => {
            let mut client = KvsClient::connect(addr)?;
            if let Some(value) = client.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        ClientCommand::Set { key, value, addr } => {
            let mut client = KvsClient::connect(addr)?;
            client.set(key, value)?;
            Ok(())
        }
        ClientCommand::Remove { key, addr } => {
            let mut client = KvsClient::connect(addr)?;
            client.remove(key)?;
            Ok(())
        }
    }
}
