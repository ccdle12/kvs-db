use crate::Result;
use std::collections::HashMap;
use std::path::Path;

/// The `KvStore` stores a key/value pair of strings.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to
/// disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> KvStore {
        let store = HashMap::new();
        KvStore { store }
    }

    /// Sets a string value according to a key string.
    ///
    /// If the key already exists, it will overwrite the value.
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        self.store.insert(key, val);
        panic!()
    }

    /// Gets the string value given a string key.
    ///
    /// Returns None, if the key doesn't exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        // Clone the value from the store.
        self.store.get(&key).cloned();
        panic!()
    }

    /// Removes a key/value pair given a string key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);
        panic!()
    }

    /// PLACEHOLDER
    pub fn open(path: &Path) -> Result<KvStore> {
        let store = HashMap::new();

        Ok(KvStore { store })
    }
}
