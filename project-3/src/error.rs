/// The custom error type for this project. Each error type will be added as an
/// enum variant.
#[derive(Fail, Debug)]
pub enum KvStoreError {
    /// Standard IO Errors.
    #[fail(display = "{}", _0)]
    IOError(#[cause] std::io::Error),
}

impl From<std::io::Error> for KvStoreError {
    fn from(err: std::io::Error) -> KvStoreError {
        KvStoreError::IOError(err)
    }
}

/// Alias for Result in this project.
pub type Result<T> = std::result::Result<T, KvStoreError>;
