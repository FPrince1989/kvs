use std::collections::HashMap;

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
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// get value by key
    pub fn get(&mut self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// remove by key
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}