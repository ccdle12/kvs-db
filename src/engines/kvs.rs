use crate::KvsEngine;
use crate::{KvStoreError, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs::{create_dir_all, remove_file, File, OpenOptions};
use std::io::prelude::*;

// TODO: TEMP
use std::io::SeekFrom;

use std::io::{BufReader, LineWriter};
use std::path::{Path, PathBuf};

// TODO: TEMP
const THRESHOLD: u16 = 10;

/// Command is an enum that represents each possible Read/Write command to the
/// DB. Each enum command will be serialized to a log file and used as the basis
/// for populatin and updating the in-memory key/value store.
#[derive(Serialize, Clone, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

/// TODO: Rename and work on this struct more
struct ByteOffset {
    start_offset: u16,
    byte_length: u16,
}

/// TODO: TEMP

/// Macro to write a command to the DB log file.
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

/// The `KvStore` stores a key/value pair of strings.
///
/// The store keeps in-memory `offset_map` recording the key and byte offset
/// of it's entry in a log file on disk. Each key/value pair is added to an
/// append only list in peristance storage on disk.
pub struct KvStore {
    /// OffsetMap is the in memory key/value store, tracking a particular key
    /// and it's byte offest.
    offset_map: BTreeMap<String, ByteOffset>,

    /// The path to the logs folder, containing the log of events for the DB.
    path_buf: PathBuf,

    /// The current byte offset of the last key/value pair. The byte offset
    /// starts from the first character in the key.
    current_offset: u16,

    /// Uncompacted bytes tracks the amount in bytes that can be compacted
    /// from the log file.
    uncompacted_bytes: u16,
}

impl KvStore {
    /// Private helper function to format the offset length. It's pretty verbose
    /// but due to the nature "Off-by-one" errors, it's preferrable to contain
    /// this logic in a function and make it extremely clear that this function
    /// is required to avoid potentially very annoying bugs.
    fn get_byte_length(cmd: &String) -> u16 {
        cmd.len() as u16 + 1
    }

    fn build_compacted_map(log_file: &File) -> Result<BTreeMap<String, Command>> {
        let mut compacted_map: BTreeMap<String, Command> = BTreeMap::new();

        for line in BufReader::new(log_file).lines() {
            let line = line?;
            let cmd: Command = serde_json::from_str(&line)?;

            if let Command::Remove { key } = &cmd {
                compacted_map.remove(key.into());
            };

            if let Command::Set { key, value } = &cmd {
                compacted_map.insert(key.into(), cmd);
            };
        }

        Ok(compacted_map)
    }

    fn create_log_file(path: &mut PathBuf) -> Result<File> {
        create_dir_all(&path)?;

        path.push("log");
        path.set_extension("txt");

        Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?)
    }

    /// Opens a connection to the Key/Value Store via a path to the log folder.
    /// If the log file doesn't exist, a file `log.txt` will be created.
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
        let log_file = Self::create_log_file(&mut path_buf)?;

        let mut offset_map = BTreeMap::new();
        let mut uncompacted_bytes = 0;
        let mut current_offset = 0;

        // Deserialize the log file into the in memory byte offset map. If the
        // uncompacted bytes cross the threshold, being the compaction process.
        for line in BufReader::new(&log_file).lines() {
            let line = line?;
            let cmd: Command = serde_json::from_str(&line)?;

            if let Command::Set { key, value } = &cmd {
                let offset = ByteOffset {
                    start_offset: current_offset,
                    byte_length: Self::get_byte_length(&line),
                };

                if let Some(v) = offset_map.insert(key.into(), offset) {
                    uncompacted_bytes += v.byte_length;
                }
            }

            if let Command::Remove { key } = &cmd {
                uncompacted_bytes += Self::get_byte_length(&line);
            }

            current_offset += Self::get_byte_length(&line);
        }

        if uncompacted_bytes > THRESHOLD {
            current_offset = 0;
            offset_map.clear();

            // TODO: Must be a better way to reuse the same file handler?
            let new_log_file_0 = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&path_buf)?;

            let compacted_map = Self::build_compacted_map(&new_log_file_0)?;

            // TODO: Actually make a copy first but we need some mechanism to
            // track the latest log file e.g. log1.txt, log2.txt etc...

            remove_file(&path_buf)?;
            let new_log_file = Self::create_log_file(&mut env::current_dir()?)?;

            for cmd in compacted_map.iter() {
                write_cmd!(&cmd.1, &new_log_file)?;

                // Repopulate the offset map with compacted entries.
                let cmd_str = serde_json::to_string(&cmd.1)?;

                if let Command::Set { key, value } = &cmd.1 {
                    let byte_length = Self::get_byte_length(&cmd_str);

                    let offset = ByteOffset {
                        start_offset: current_offset,
                        byte_length,
                    };

                    offset_map.insert(key.into(), offset);

                    current_offset += byte_length;
                };
            }
        }

        Ok(KvStore {
            offset_map,
            path_buf,
            current_offset,
            uncompacted_bytes,
        })
    }

    /// Private helper function to return a file handler as read only to the
    /// DB log file.
    fn log_file_write(&self) -> Result<File> {
        Ok(OpenOptions::new().append(true).open(&self.path_buf)?)
    }

    /// Private helper function to return a file handler as write only to the
    /// DB log file.
    fn log_file_read(&self) -> Result<File> {
        Ok(OpenOptions::new().read(true).open(&self.path_buf)?)
    }
}

impl KvsEngine for KvStore {
    /// Retrieves the value of the key/pair given a key as an arguement.
    ///
    /// Raises a KeyNotFoundError if it doesn't exist.
    /// TODO: We only need to write SET commands to the DB./
    /// TODO: rename byte_length to cmd_byte_length
    fn get(&self, key: String) -> Result<Option<String>> {
        if let Some(v) = self.offset_map.get(&key) {
            let mut reader = BufReader::new(self.log_file_read()?);

            let cursor = SeekFrom::Start(v.start_offset.into());
            reader.seek(cursor)?;

            let mut buffer = vec![0u8; v.byte_length.into()];
            reader.read_exact(&mut buffer)?;

            if let Command::Set { key, value } = serde_json::from_slice(&buffer)? {
                return Ok(Some(value));
            }
        }

        Err(KvStoreError::KeyNotFoundError)
    }

    /// Sets a string value according to a key.
    /// If the key already exists the value will be overwritten.
    ///
    ///
    /// TODO: Figure out the failing doc test that has been removed. Use the
    /// course-examples/ for reference.
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_cmd = Command::Set {
            key: key.clone(),
            value,
        };
        write_cmd!(&set_cmd, self.log_file_write()?)?;

        let cmd = serde_json::to_string(&set_cmd)?;
        let offset = ByteOffset {
            start_offset: self.current_offset,
            byte_length: Self::get_byte_length(&cmd),
        };

        if let Some(duplicate_key) = self.offset_map.insert(key, offset) {
            self.uncompacted_bytes += duplicate_key.byte_length;
        };

        self.current_offset += Self::get_byte_length(&cmd);

        Ok(())
    }

    /// Removes a key/value pair given a string key.
    fn remove(&mut self, key: String) -> Result<()> {
        let cmd = Command::Remove { key };

        write_cmd!(&cmd, &self.log_file_write()?)?;

        if let Command::Remove { key } = cmd {
            match self.offset_map.remove(&key) {
                Some(_x) => return Ok(()),
                None => return Err(KvStoreError::KeyNotFoundError),
            }
        };

        Ok(())
    }
}
