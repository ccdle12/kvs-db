use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::{Path, PathBuf};

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
///
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
pub struct KvStore {
    /// Store is the in memory key/value store.
    store: HashMap<String, String>,

    /// The path to the logs folder, containing the log of events for the DB.
    path_buf: PathBuf,
}

impl KvStore {
    /// Opens a connection to the Key/Value Store via a path to the log folder.
    /// If the log file don't exist, the file `log.txt` will be created.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use kvs::KvStore;
    /// # use fs::env;
    /// let mut kv_store = KvStore::open(&env::curent_dir()?)?;
    /// ```
    pub fn open(path: &Path) -> Result<KvStore> {
        create_dir_all(&path)?;

        let mut path_buf = PathBuf::from(&path);
        path_buf.push("log");
        path_buf.set_extension("txt");

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path_buf)
            .expect("failed to create file using path_buf");

        Ok(KvStore {
            store: HashMap::new(),
            path_buf,
        })
    }

    /// Sets a string value according to a key.
    /// If the key already exists the value will be overwritten.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use kvs::KvStore;
    /// # use fs::env;
    /// ...
    ///
    /// let key = "hello";
    /// let value = "world";
    ///
    /// let mut kv_store = KvStore::open(&env::current_dir()?)?;
    /// kv_store.set(key, value)?;
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let file_handler = OpenOptions::new()
            .append(true)
            .open(&self.path_buf)
            .expect("failed to open path_buf when setting key/value.");

        serde_json::to_writer(&file_handler, &Command::Set { key, value })?;

        Ok(())
    }

    /// Retrieves the value of the key/pair given a key as an arguement.
    ///
    /// Returns None, if the key doesn't exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        // Clone the value from the store.
        self.store.get(&key).cloned();
        panic!()
    }

    /// Removes a key/value pair given a string key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let file = File::create("log.txt")?;
        let remove_cmd = Command::Remove { key };

        serde_json::to_writer(file, &remove_cmd)?;
        Ok(())
    }
}

/// PLACEHOLDER
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
