use std::collections::HashMap;
use std::path::Path;
use crate::Result;

/// A key-value store
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// init
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// set key and value
    pub fn set(&mut self, key: String, value: String)-> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    /// get value by key
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    /// remove by key
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    /// open the database file
    pub fn open(path: &Path) -> Result<KvStore> {
        Ok(KvStore {
            map: HashMap::new(),
        })
    }
}
