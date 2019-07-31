use crate::{KvStoreError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::remove_file;
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
        let path_buf = create_log_file(path)?;
        let file_handler = open_file_all_permissions(&path_buf)?;

        let store = unpack_log_file(file_handler)?;
        let history = compact_history(&store);

        write_compacted_history(history, &path_buf)?;

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
        write_cmd(&set_cmd, open_file_append_only(&self.path_buf)?)?;

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

        match value {
            Some(v) => Ok(Some(v)),
            None => Err(KvStoreError::KeyNotFoundError),
        }
    }

    /// Removes a key/value pair given a string key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let cmd = Command::Remove { key };
        write_cmd(&cmd, open_file_append_only(&self.path_buf)?)?;

        if let Command::Remove { key } = &cmd {
            match self.store.remove(&key.to_string()) {
                Some(_x) => return Ok(()),
                None => return Err(KvStoreError::KeyNotFoundError),
            }
        };

        Ok(())
    }
}

/// Private helper function to write a command to the log file.
fn write_cmd(cmd: &Command, file_handler: impl Write) -> Result<()> {
    let mut cmd_serialized = serde_json::to_string(&cmd)?;
    cmd_serialized.push('\n');

    LineWriter::new(file_handler).write(cmd_serialized.as_bytes())?;

    Ok(())
}

/// Creates the log file for the `write-ahead-log` given a file path.
fn create_log_file(path: &Path) -> Result<PathBuf> {
    let mut path_buf = PathBuf::from(path);
    create_dir_all(&path_buf)?;

    path_buf.push("log");
    path_buf.set_extension("txt");

    Ok(path_buf)
}

/// Opens a file as append only given a path to the file.
fn open_file_append_only(path: &PathBuf) -> Result<File> {
    let file_handler = OpenOptions::new().append(true).open(path)?;

    Ok(file_handler)
}

/// Opens a file with read/write permissions and creates the file it doesn't
/// exist.
fn open_file_all_permissions(path: &PathBuf) -> Result<File> {
    let file_handler = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    Ok(file_handler)
}

/// Function that unpacks a log file given a file handler and returns an in-memory
/// k/v store with the read logs.
fn unpack_log_file(file_handler: impl Read) -> Result<HashMap<String, String>> {
    // Create the kv store.
    let mut store = HashMap::new();

    // Opens the log file and reads each line. The logs are deserialized
    // and updated in a returned in-memory store.
    for line in BufReader::new(file_handler).lines() {
        let cmd: Command = serde_json::from_str(&line?)?;

        if let Command::Set { key, value } = &cmd {
            store.insert(key.to_string(), value.to_string());
        };

        if let Command::Remove { key } = &cmd {
            store.remove(&key.to_string());
        };
    }

    Ok(store)
}

/// A helper function that removes redundant entries in the log, returns it as
/// a vector of commands with the intention of writing it to the log file.
fn compact_history(store: &HashMap<String, String>) -> Vec<Command> {
    let mut compacted_history: Vec<Command> = Vec::new();

    for (key, value) in store {
        let cmd = Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        };

        compacted_history.push(cmd);
    }

    compacted_history
}

/// A helper function that receives a compacted history and path to the log file.
/// It removes the logfile and rewrites it with the compacted history.
fn write_compacted_history(history: Vec<Command>, path: &PathBuf) -> Result<()> {
    remove_file(path)?;
    let file_handler = open_file_all_permissions(path)?;

    history
        .into_iter()
        .for_each(|x| write_cmd(&x, &file_handler).unwrap());

    Ok(())
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
