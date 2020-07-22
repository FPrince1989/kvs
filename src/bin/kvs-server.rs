use kvs::{KvsServerOpt, Result};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = KvsServerOpt::from_args();
    println!("{:?}", opt);
    Ok(())
}
