use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::{Command, KvsError, Result};

/// A key-value store
pub struct KvStore {
    map: HashMap<String, String>,
    _path: PathBuf,
    _reader: BufReader<File>,
    writer: BufWriter<File>,
}

impl KvStore {
    /// set key and value
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key.to_owned(), value.to_owned());
        let cmd = Command::Set { key, value };
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.write_all("\n".as_bytes())?;
        Ok(())
    }

    /// get value by key
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let value_opt = self.map.get(&key);
        Ok(value_opt.cloned())
    }

    /// remove by key
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.map.remove(&key).is_some() {
            let cmd = Command::Remove { key };
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.write_all("\n".as_bytes())?;
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

    /// open and load the database file
    pub fn open(path: &Path) -> Result<KvStore> {
        let mut path = path.to_path_buf();
        path.push("kvs.log");
        let writer = BufWriter::new(OpenOptions::new().create(true).append(true).open(&path)?);
        let mut reader = BufReader::new(OpenOptions::new().read(true).open(&path)?);
        let mut map = HashMap::new();
        let mut cmd_str = String::new();
        while let Ok(_) = reader.read_line(&mut cmd_str) {
            if cmd_str.is_empty() {
                break;
            }
            let cmd = serde_json::from_str(cmd_str.as_str())?;
            match cmd {
                Command::Set { key, value } => {
                    map.insert(key, value);
                }
                Command::Remove { key } => {
                    map.remove(&key);
                }
                _ => unreachable!(),
            }
            cmd_str.clear();
        }
        Ok(KvStore {
            map,
            _path: path,
            _reader: reader,
            writer,
        })
    }
}
