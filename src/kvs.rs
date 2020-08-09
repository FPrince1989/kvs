use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde::Serialize;

use crate::{KvsEngine, KvsError, Result};

const MAX_REDUNDANT_COUNT: u64 = 1000;
const DATA_FILENAME: &str = "kvs.data";
const COMPACT_FILENAME: &str = "kvs-compact.data";

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

/// A key-value store
pub struct KvStore {
    pos_map: HashMap<String, u64>,
    dir: PathBuf,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    write_pos: u64,
    redundant_count: u64,
}

impl KvStore {
    /// open and load the database file
    pub fn open(dir: &Path) -> Result<KvStore> {
        let mut file_path = dir.to_path_buf();
        file_path.push(DATA_FILENAME);
        let writer = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_path)?,
        );
        let mut reader = BufReader::new(OpenOptions::new().read(true).open(&file_path)?);
        let mut map = HashMap::new();
        let mut cmd_str = String::new();
        let mut last_index = 0 as u64;
        let mut redundant_count = 0 as u64;

        while let Ok(_) = reader.read_line(&mut cmd_str) {
            if cmd_str.is_empty() {
                break;
            }
            let cmd: Command = serde_json::from_str(cmd_str.as_str())?;
            match cmd {
                Command::Set { key, .. } => {
                    if map.insert(key, last_index).is_some() {
                        redundant_count += 1;
                    }
                }
                Command::Remove { key } => {
                    if map.remove(&key).is_some() {
                        redundant_count += 1;
                    }
                }
            }

            last_index += cmd_str.len() as u64;
            cmd_str.clear();
        }
        Ok(KvStore {
            pos_map: map,
            dir: dir.to_path_buf(),
            reader,
            writer,
            write_pos: last_index,
            redundant_count,
        })
    }

    fn compact(&mut self) -> Result<()> {
        let mut compact_file_path = self.dir.to_path_buf();
        compact_file_path.push(COMPACT_FILENAME);
        let mut file_path = self.dir.to_path_buf();
        file_path.push(DATA_FILENAME);
        let mut compact_writer = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&compact_file_path)?,
        );
        let compact_reader =
            BufReader::new(OpenOptions::new().read(true).open(&compact_file_path)?);
        let mut compact_pos_map = HashMap::new();
        let mut last_index = 0 as u64;
        for (key, &pos) in self.pos_map.iter() {
            compact_pos_map.insert(key.clone(), last_index);
            self.reader.seek(SeekFrom::Start(pos))?;
            let mut cmd_str = String::new();
            self.reader.read_line(&mut cmd_str)?;
            compact_writer.write_all(cmd_str.as_bytes())?;
            last_index += cmd_str.len() as u64;
        }
        compact_writer.flush()?;

        self.writer = compact_writer;
        self.pos_map = compact_pos_map;
        self.redundant_count = 0;
        self.write_pos = last_index;
        self.reader = compact_reader;

        fs::rename(compact_file_path, file_path)?;

        Ok(())
    }
}

impl KvsEngine for KvStore {
    /// set key and value
    fn set(&mut self, key: String, value: String) -> Result<()> {
        if self
            .pos_map
            .insert(key.to_owned(), self.write_pos)
            .is_some()
        {
            self.redundant_count += 1;
        }
        let cmd = Command::Set { key, value };
        let mut cmd_str = serde_json::to_string(&cmd)?;
        cmd_str.push('\n');
        self.writer.write_all(cmd_str.as_bytes())?;
        self.writer.flush()?;
        self.write_pos += cmd_str.len() as u64;

        if self.redundant_count > MAX_REDUNDANT_COUNT {
            self.compact()?;
        }

        Ok(())
    }

    /// get value by key
    fn get(&mut self, key: String) -> Result<Option<String>> {
        let value_opt = self.pos_map.get(&key);
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
    fn remove(&mut self, key: String) -> Result<()> {
        if self.pos_map.remove(&key).is_some() {
            let cmd = Command::Remove { key };
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.write_all(b"\n")?;
            self.writer.flush()?;
            self.redundant_count += 1;
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

    /// engine's name
    fn name(&self) -> String {
        "kvs".to_string()
    }
}
