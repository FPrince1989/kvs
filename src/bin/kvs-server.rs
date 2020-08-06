use structopt::StructOpt;

use kvs::{KvStore, KvsServer, KvsServerOpt, Result};

fn main() -> Result<()> {
    let opt = KvsServerOpt::from_args();
    let engine = KvStore::open(std::env::current_dir()?.as_path())?;
    let mut server = KvsServer::new(engine);
    server.run(opt.addr)?;

    Ok(())
}
