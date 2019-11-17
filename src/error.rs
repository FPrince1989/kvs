use failure::Fail;

/// Kvs Error Enum
#[derive(Fail, Debug)]
pub enum KvsError {
    /// Can not find the key in kvs
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// io Error
    #[fail(display = "{}", _0)]
    IoErr(#[cause] std::io::Error),
    /// serde Error
    #[fail(display = "{}", _0)]
    SerdeErr(#[cause] serde_json::error::Error),
}

impl From<std::io::Error> for KvsError {
    fn from(err: std::io::Error) -> Self {
        KvsError::IoErr(err)
    }
}

impl From<serde_json::error::Error> for KvsError {
    fn from(err: serde_json::error::Error) -> Self {
        KvsError::SerdeErr(err)
    }
}

/// wrap the KvsError Result to a simple type
pub type Result<T> = std::result::Result<T, KvsError>;
