use crate::kv_trait::KVStore;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MemoryStore {
    data: HashMap<String, String>,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore {
            data: HashMap::new(),
        }
    }
}

impl KVStore for MemoryStore {
    fn set(&mut self, key: String, value: String) -> Result<Option<String>> {
        let last_val = self.data.remove(&key);
        self.data.insert(key, value);
        Ok(last_val)
    }

    fn get(&self, key: &str) -> Result<Option<&String>> {
        Ok(self.data.get(key))
    }

    fn remove(&mut self, key: &str) -> Result<Option<String>> {
        Ok(self.data.remove(key))
    }
}
