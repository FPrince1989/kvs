use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use serde_json::Deserializer;
use slog::{debug, error, info};

use crate::thread_pool::ThreadPool;
use crate::{KvsEngine, Request, Response, Result, LOGGING};

pub struct KvsServer<T: KvsEngine, P: ThreadPool> {
    engine: T,
    pool: P,
}

impl<T: KvsEngine, P: ThreadPool> KvsServer<T, P> {
    pub fn new(engine: T, pool: P) -> Self {
        KvsServer { engine, pool }
    }

    pub fn run(&mut self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        info!(
            LOGGING.logger,
            "Init Listener Success, Version:{}, Engine Name:{}, Listening On:{}",
            env!("CARGO_PKG_VERSION"),
            self.engine.name(),
            addr
        );
        for stream in listener.incoming() {
            let engine = self.engine.clone();
            self.pool.spawn(move || match stream {
                Ok(tcp_stream) => {
                    if let Err(e) = handle_stream(engine, tcp_stream) {
                        error!(LOGGING.logger, "Error: {}", e);
                    }
                }
                Err(e) => {
                    error!(LOGGING.logger, "Network Error: {}", e);
                }
            })
        }

        Ok(())
    }
}

fn handle_stream<T: KvsEngine>(engine: T, stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let request_reader = Deserializer::from_reader(reader).into_iter::<Request>();

    for request in request_reader {
        let request = request?;
        debug!(LOGGING.logger, "Request Content: {:?}", request);
        match request {
            Request::Get { key } => match engine.get(key) {
                Ok(value) => {
                    serde_json::to_writer(&mut writer, &Response::Ok(value))?;
                    writer.flush()?;
                }
                Err(e) => {
                    serde_json::to_writer(&mut writer, &Response::Err(e.to_string()))?;
                    writer.flush()?;
                }
            },
            Request::Set { key, value } => match engine.set(key, value) {
                Ok(_) => {
                    serde_json::to_writer(&mut writer, &Response::Ok(None))?;
                    writer.flush()?;
                }
                Err(e) => {
                    serde_json::to_writer(&mut writer, &Response::Err(e.to_string()))?;
                    writer.flush()?;
                }
            },
            Request::Remove { key } => match engine.remove(key) {
                Ok(_) => {
                    serde_json::to_writer(&mut writer, &Response::Ok(None))?;
                    writer.flush()?;
                }
                Err(e) => {
                    serde_json::to_writer(&mut writer, &Response::Err(e.to_string()))?;
                    writer.flush()?;
                }
            },
        }
    }
    Ok(())
}
