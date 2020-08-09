use std::env;
use std::net::SocketAddr;
use std::process::exit;

use structopt::StructOpt;

use kvs::{Engine, KvStore, KvsEngine, KvsServer, KvsServerOpt, Result, SledKvsEngine};

fn main() -> Result<()> {
    let opt = KvsServerOpt::from_args();
    match opt.engine {
        Engine::Kvs => {
            run_with_engine(KvStore::open(std::env::current_dir()?.as_path())?, opt.addr)
        }

        Engine::Sled => run_with_engine(
            SledKvsEngine::new(sled::open(env::current_dir()?)?),
            opt.addr,
        ),
    }
}

fn run_with_engine<T: KvsEngine>(engine: T, addr: SocketAddr) -> Result<()> {
    let mut server = KvsServer::new(engine);
    match server.run(addr) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    }
}
