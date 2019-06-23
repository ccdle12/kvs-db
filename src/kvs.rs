pub struct KvStore {
    store: std::collections::HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        let store = std::collections::HashMap::new();
        KvStore { store }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.store.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<String> {
        // Clone the value from the store.
        self.store.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
