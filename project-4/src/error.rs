/// The custom error type for this project. Each error type will be added as an
/// enum variant.
#[derive(Fail, Debug)]
pub enum KvStoreError {
    /// Standard IO Errors.
    #[fail(display = "{}", _0)]
    IOError(#[cause] std::io::Error),

    /// Serde Json Serialization Errors.
    #[fail(display = "{}", _0)]
    SerdeError(#[cause] serde_json::Error),

    /// Error for a key not found in the key value store.
    #[fail(display = "Key not found")]
    KeyNotFoundError,

    /// A error with a String message.
    #[fail(display = "{}", _0)]
    StringError(String),

    /// Sled DB errors.
    #[fail(display = "{}", _0)]
    SledError(sled::Error),

    /// FromStringUtf8 Error when converting a Vec<u8> to String.
    #[fail(display = "{}", _0)]
    StringUtf8Error(#[cause] std::string::FromUtf8Error),
}

impl From<std::io::Error> for KvStoreError {
    fn from(err: std::io::Error) -> KvStoreError {
        KvStoreError::IOError(err)
    }
}

impl From<serde_json::Error> for KvStoreError {
    fn from(err: serde_json::Error) -> KvStoreError {
        KvStoreError::SerdeError(err)
    }
}

impl From<sled::Error> for KvStoreError {
    fn from(err: sled::Error) -> KvStoreError {
        KvStoreError::SledError(err)
    }
}

impl From<std::string::FromUtf8Error> for KvStoreError {
    fn from(err: std::string::FromUtf8Error) -> KvStoreError {
        KvStoreError::StringUtf8Error(err)
    }
}

/// Alias for Result in this project.
pub type Result<T> = std::result::Result<T, KvStoreError>;
