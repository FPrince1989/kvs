use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::{Command, KvsError, Result};

/// A key-value store
pub struct KvStore {
    map: HashMap<String, u64>,
    _path: PathBuf,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    write_pos: u64,
}

impl KvStore {
    /// set key and value
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key.to_owned(), self.write_pos);
        let cmd = Command::Set { key, value };
        let mut cmd_str = serde_json::to_string(&cmd)?;
        cmd_str.push('\n');
        self.writer.write_all(cmd_str.as_bytes())?;
        self.writer.flush()?;
        self.write_pos += cmd_str.len() as u64;
        Ok(())
    }

    /// get value by key
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let value_opt = self.map.get(&key);
        if let Some(&pos) = value_opt {
            self.reader.seek(SeekFrom::Start(pos))?;
            let mut cmd_str = String::new();
            self.reader.read_line(&mut cmd_str)?;
            let cmd = serde_json::from_str::<Command>(cmd_str.as_str())?;
            match cmd {
                Command::Set { value, .. } => Ok(Some(value)),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
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
        let mut last_index = 0 as u64;

        while let Ok(_) = reader.read_line(&mut cmd_str) {
            if cmd_str.is_empty() {
                break;
            }
            let cmd: Command = serde_json::from_str(cmd_str.as_str())?;
            match cmd {
                Command::Set { key, .. } => {
                    map.insert(key, last_index);
                }
                Command::Remove { key } => {
                    map.remove(&key);
                }
                _ => unreachable!(),
            }

            last_index += cmd_str.len() as u64;
            cmd_str.clear();
        }
        Ok(KvStore {
            map,
            _path: path,
            reader,
            writer,
            write_pos: last_index,
        })
    }
}
