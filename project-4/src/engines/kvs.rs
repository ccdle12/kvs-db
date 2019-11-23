use super::KvsEngine;
use crate::{KvStoreError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Command is an enum with each possible command of the database. Each enum
/// command will be serialized to a log file and used as the basis for populating/
/// updating an in-memory key/value store.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

/// The `KvStore` stores a key/value pair of strings.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to
/// disk.
#[derive(Clone)]
pub struct KvStore {
    /// Store is the in memory key/value store.
    store: Arc<Mutex<HashMap<String, String>>>,

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
    /// # use std::env;
    ///
    /// let current_dir = env::current_dir().unwrap();
    /// let mut kv_store = KvStore::open(&current_dir).unwrap();
    /// ```
    pub fn open(path: &Path) -> Result<KvStore> {
        let mut path_buf = PathBuf::from(path);
        create_dir_all(&path_buf)?;

        path_buf.push("log");
        path_buf.set_extension("txt");

        let file_handler = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path_buf)?;

        // Create the kv store.
        let mut store = HashMap::new();

        // Opens the log file and reads each line. The logs are deserialized
        // and updated in the in-memory store.
        for line in BufReader::new(file_handler).lines() {
            let cmd: Command = serde_json::from_str(&line?)?;

            if let Command::Set { key, value } = &cmd {
                store.insert(key.to_string(), value.to_string());
            };

            if let Command::Remove { key } = &cmd {
                store.remove(&key.to_string());
            };
        }

        Ok(KvStore {
            store: Arc::new(Mutex::new(store)),
            path_buf,
        })
    }

    /// Private helper function to return a file handler as read only to the log.
    fn log_file_append_only(&self) -> Result<File> {
        Ok(OpenOptions::new().append(true).open(&self.path_buf)?)
    }
}

macro_rules! write_cmd {
    ($command:expr, $file_handler:expr) => {{
        let c = $command;
        let f = $file_handler;

        let mut cmd = serde_json::to_string(&c)?;
        cmd.push('\n');

        LineWriter::new(f).write(cmd.as_bytes())?;

        Ok(())
    } as Result<()>};
}

/// KvsEngine trait implementation on the concrete struct KvStore.
impl KvsEngine for KvStore {
    /// Retrieves the value of the key/pair given a key as an arguement.
    ///
    /// Returns None, if the key doesn't exist.
    fn get(&self, key: String) -> Result<Option<String>> {
        // Clone the value from the store.
        match self.store.lock().unwrap().get(&key).cloned() {
            Some(v) => Ok(Some(v)),
            None => Err(KvStoreError::KeyNotFoundError),
        }
    }

    /// Sets a string value according to a key.
    /// If the key already exists the value will be overwritten.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use kvs::KvStore;
    /// # use std::env;
    /// let key = "hello";
    /// let value = "world";
    ///
    /// let current_dir = env::current_dir().unwrap();
    /// let mut kv_store = KvStore::open(&current_dir).unwrap();
    /// kv_store.set(key.to_string(), value.to_string()).unwrap();
    /// ```
    fn set(&self, key: String, value: String) -> Result<()> {
        let set_cmd = Command::Set { key, value };
        write_cmd!(&set_cmd, self.log_file_append_only()?)?;

        if let Command::Set { key, value } = set_cmd {
            self.store.lock().unwrap().insert(key, value);
        }

        Ok(())
    }

    /// Removes a key/value pair given a string key.
    fn remove(&self, key: String) -> Result<()> {
        let cmd = Command::Remove { key };
        write_cmd!(&cmd, &self.log_file_append_only()?)?;

        if let Command::Remove { key } = cmd {
            match self.store.lock().unwrap().remove(&key) {
                Some(_x) => return Ok(()),
                None => return Err(KvStoreError::KeyNotFoundError),
            }
        };

        Ok(())
    }
}
