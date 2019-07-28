use crate::{KvStoreError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::path::{Path, PathBuf};

/// The `KvStore` stores a key/value pair of strings.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to
/// disk.
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
            .open(&path_buf)
            .expect("failed to create file using path_buf");

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

        Ok(KvStore { store, path_buf })
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
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_cmd = Command::Set { key, value };
        self.write_cmd(&set_cmd, self.log_file_append_only()?)?;

        if let Command::Set { key, value } = set_cmd {
            self.store.insert(key, value);
        }

        Ok(())
    }

    /// Retrieves the value of the key/pair given a key as an arguement.
    ///
    /// Returns None, if the key doesn't exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        // Clone the value from the store.
        let value = self.store.get(&key).cloned();
        self.write_cmd(&Command::Get { key }, self.log_file_append_only()?)?;

        match value {
            Some(v) => Ok(Some(v)),
            None => Err(KvStoreError::KeyNotFoundError),
        }
    }

    /// Removes a key/value pair given a string key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let cmd = Command::Remove { key };
        self.write_cmd(&cmd, self.log_file_append_only()?)?;

        if let Command::Remove { key } = cmd {
            match self.store.remove(&key) {
                Some(_x) => return Ok(()),
                None => return Err(KvStoreError::KeyNotFoundError),
            }
        };

        Ok(())
    }

    /// Private helper function to write a command to the log file.
    fn write_cmd(&self, cmd: &Command, file_handler: impl Write) -> Result<()> {
        let mut cmd_serialized = serde_json::to_string(&cmd)?;
        cmd_serialized.push('\n');

        LineWriter::new(file_handler).write(cmd_serialized.as_bytes())?;

        Ok(())
    }

    /// Private helper function to return a file handler as read only to the log.
    fn log_file_append_only(&self) -> Result<File> {
        let file_handler = OpenOptions::new().append(true).open(&self.path_buf)?;
        Ok(file_handler)
    }
}

/// Command is an enum with each possible command of the database. Each enum
/// command will be serialized to a log file and used as the basis for populating/
/// updating an in-memory key/value store.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}
