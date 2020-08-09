use failure::Fail;
use std::string::FromUtf8Error;

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
    /// string Error
    #[fail(display = "{}", _0)]
    StringErr(String),
    /// Sled error
    #[fail(display = "sled error: {}", _0)]
    SledErr(#[cause] sled::Error),
    /// String FromUtf8Error error
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8Err(#[cause] FromUtf8Error),
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

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> Self {
        KvsError::SledErr(err)
    }
}

impl From<FromUtf8Error> for KvsError {
    fn from(err: FromUtf8Error) -> Self {
        KvsError::Utf8Err(err)
    }
}

/// wrap the KvsError Result to a simple type
pub type Result<T> = std::result::Result<T, KvsError>;
