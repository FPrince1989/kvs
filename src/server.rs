use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use serde_json::Deserializer;
use slog::{debug, error, info, Logger};
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::Severity;
use sloggers::Build;

use crate::{KvsEngine, Request, Response, Result};

pub struct KvsServer<T: KvsEngine> {
    engine: T,
    logger: Logger,
}

impl<T: KvsEngine> KvsServer<T> {
    pub fn new(engine: T) -> Self {
        let mut builder = TerminalLoggerBuilder::new();
        builder.level(Severity::Debug);
        builder.destination(Destination::Stderr);
        let logger = builder.build().unwrap();
        KvsServer { engine, logger }
    }

    pub fn run(&mut self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        info!(
            self.logger,
            "Init Listener Success, Version:{}, Engine Name:{}, Listening On:{}",
            env!("CARGO_PKG_VERSION"),
            self.engine.name(),
            addr
        );
        for stream in listener.incoming() {
            match stream {
                Ok(tcp_stream) => {
                    self.handle_stream(tcp_stream)?;
                }
                Err(e) => {
                    error!(self.logger, "Network Error: {}", e);
                }
            }
        }

        Ok(())
    }

    fn handle_stream(&mut self, stream: TcpStream) -> Result<()> {
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let request_reader = Deserializer::from_reader(reader).into_iter::<Request>();

        for request in request_reader {
            let request = request?;
            debug!(self.logger, "Request Content: {:?}", request);
            match request {
                Request::Get { key } => match self.engine.get(key) {
                    Ok(value) => {
                        serde_json::to_writer(&mut writer, &Response::Ok(value))?;
                        writer.flush()?;
                    }
                    Err(e) => {
                        serde_json::to_writer(&mut writer, &Response::Err(e.to_string()))?;
                        writer.flush()?;
                    }
                },
                Request::Set { key, value } => match self.engine.set(key, value) {
                    Ok(_) => {
                        serde_json::to_writer(&mut writer, &Response::Ok(None))?;
                        writer.flush()?;
                    }
                    Err(e) => {
                        serde_json::to_writer(&mut writer, &Response::Err(e.to_string()))?;
                        writer.flush()?;
                    }
                },
                Request::Remove { key } => match self.engine.remove(key) {
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
}
