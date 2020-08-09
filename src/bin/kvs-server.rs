use std::net::SocketAddr;
use std::process::exit;
use std::{env, fs};

use structopt::StructOpt;

use kvs::{Engine, KvStore, KvsEngine, KvsServer, KvsServerOpt, Result, SledKvsEngine};
use std::env::current_dir;

const ENGINE_FILE_NAME: &str = "engine.data";

fn main() -> Result<()> {
    let opt = KvsServerOpt::from_args();
    let current_engine = get_current_engine()?;
    if current_engine.is_some() && current_engine.unwrap() != opt.engine {
        eprintln!("Wrong engine");
        exit(1);
    }
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
    fs::write(current_dir()?.join(ENGINE_FILE_NAME), engine.name())?;
    let mut server = KvsServer::new(engine);
    match server.run(addr) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    }
}

fn get_current_engine() -> Result<Option<Engine>> {
    let engine_file = current_dir()?.join(ENGINE_FILE_NAME);
    if !engine_file.exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine_file)?.parse() {
        Ok(engine) => Ok(Some(engine)),
        Err(e) => {
            eprintln!("Wrong Engine Type: {}", e);
            Ok(None)
        }
    }
}
