use failure::Fail;

/// Kvs Error Enum
#[derive(Fail, Debug)]
pub enum KvsError {
    /// Can not find the key in kvs
    #[fail(display = "Key not found")]
    KeyNotFound,
}

/// wrap the KvsError Result to a simple type
pub type Result<T> = std::result::Result<T, KvsError>;