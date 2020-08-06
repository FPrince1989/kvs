use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};

use serde::Deserialize;
use serde_json::de::IoRead;
use serde_json::Deserializer;

use crate::{KvsError, Request, Response, Result};

pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    pub fn connect(addr: SocketAddr) -> Result<Self> {
        let reader = TcpStream::connect(addr)?;
        let writer = reader.try_clone()?;
        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(reader)),
            writer: BufWriter::new(writer),
        })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;
        let response = Response::deserialize(&mut self.reader)?;
        match response {
            Response::Ok(value) => Ok(value),
            Response::Err(e) => Err(KvsError::StringErr(e)),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;
        let response = Response::deserialize(&mut self.reader)?;
        match response {
            Response::Ok(_) => Ok(()),
            Response::Err(e) => Err(KvsError::StringErr(e)),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;
        let response = Response::deserialize(&mut self.reader)?;
        match response {
            Response::Ok(_) => Ok(()),
            Response::Err(e) => Err(KvsError::StringErr(e)),
        }
    }
}
