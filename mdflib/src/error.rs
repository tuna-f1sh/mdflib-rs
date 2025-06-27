//! Error types for mdflib operations

use thiserror::Error;

/// Error types that can occur when working with MDF files
#[derive(Error, Debug)]
pub enum MdfError {
    #[error("Failed to open file: {0}")]
    FileOpen(String),

    #[error("Failed to read header")]
    HeaderRead,

    #[error("Failed to read measurement info")]
    MeasurementInfo,

    #[error("Failed to read data")]
    DataRead,

    #[error("Failed to initialize measurement")]
    MeasurementInit,

    #[error("Failed to finalize measurement")]
    MeasurementFinalize,

    #[error("Invalid file format")]
    InvalidFormat,

    #[error("Null pointer encountered")]
    NullPointer,

    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),

    #[error("Invalid channel type: {0}")]
    InvalidChannelType(u8),

    #[error("Invalid data type: {0}")]
    InvalidDataType(u8),

    #[error("Buffer too small: needed {needed}, got {actual}")]
    BufferTooSmall { needed: usize, actual: usize },

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("String conversion error: {0}")]
    StringConversion(#[from] std::ffi::NulError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for mdflib operations
pub type Result<T> = std::result::Result<T, MdfError>;
