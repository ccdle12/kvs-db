/// The custom error type for this project. Each different error type will be
/// added as an enum variant.
#[derive(Fail, Debug)]
pub enum KvStoreError {
    /// Used for errors that are miscellaneous and/or cannot be explained.
    #[fail(display = "An unknown error has occurred")]
    UnknownError,

    /// Error for a key not found in the key value store.
    #[fail(display = "Key not found")]
    KeyNotFoundError,

    /// Serde serialization and deserialization errors.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    /// Standard Input/Output errors.
    #[fail(display = "{}", _0)]
    IOError(#[cause] std::io::Error),
}

impl From<serde_json::Error> for KvStoreError {
    fn from(err: serde_json::Error) -> KvStoreError {
        KvStoreError::Serde(err)
    }
}

impl From<std::io::Error> for KvStoreError {
    fn from(err: std::io::Error) -> KvStoreError {
        KvStoreError::IOError(err)
    }
}

/// Shorthand alias for Result in this project, uses the concrete implementation
/// of KvStoreError.
pub type Result<T> = std::result::Result<T, KvStoreError>;
