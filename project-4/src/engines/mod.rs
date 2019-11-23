//! This module provies the key value storage engines.

use crate::Result;

/// Trait (interface) for the key value storage engine.
pub trait KvsEngine: Clone + Send + 'static {
    /// Sets value of a key - all strings.
    ///
    /// If the key already exists then the value will be overwritten.
    fn set(&self, key: String, value: String) -> Result<()>;

    /// Gets the value of a given key.
    ///
    /// Returns `None` if the key does not exist.
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// An error `KvsError::KeyNotFound` is returned if a key does not exist.
    fn remove(&self, key: String) -> Result<()>;
}

mod kvs;
mod sled;

pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;
