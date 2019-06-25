/// The custom error type for this project. Each different error type will be
/// added as an enum variant.
#[derive(Fail, Debug)]
pub enum KvStoreError {
    /// Used for errors that are miscellaneous and/or cannot be explained.
    #[fail(display = "An unknown error has occurred")]
    UnknownError,
}

/// Shorthand alias for Result in this project, uses the concrete implementation
/// of KvStoreError.
pub type Result<T> = std::result::Result<T, KvStoreError>;
