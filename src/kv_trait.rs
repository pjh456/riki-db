use anyhow::Result;

pub trait KVStore {
    fn set(&mut self, key: String, value: String) -> Result<Option<String>>;

    fn get(&self, key: &str) -> Result<Option<&String>>;

    fn remove(&mut self, key: &str) -> Result<Option<String>>;

    fn scan_prefix(&self, prefix: &str) -> Result<Vec<(String, String)>> {
        Ok(Vec::new())
    }
}
