use std::io;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("config error: {0}")]
    ServiceError(#[from] service_utils_rs::error::Error),

    #[error("Other system error: {0}")]
    SystemError(String),
}

pub type Result<T> = core::result::Result<T, Error>;
