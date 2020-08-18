use sled::Db;

use crate::Result;
use crate::{KvsEngine, KvsError};

#[derive(Clone)]
pub struct SledKvsEngine(Db);

impl SledKvsEngine {
    pub fn new(db: Db) -> Self {
        SledKvsEngine(db)
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&self, key: String, value: String) -> Result<()> {
        self.0.insert(key, value.into_bytes())?;
        self.0.flush()?;
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self
            .0
            .get(key)?
            .map(|v| v.to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&self, key: String) -> Result<()> {
        self.0.remove(key)?.ok_or(KvsError::KeyNotFound)?;
        self.0.flush()?;
        Ok(())
    }

    fn name(&self) -> String {
        "sled".to_string()
    }
}
