//! Error types for mdflib operations

use thiserror::Error;

/// Error types that can occur when working with MDF files
#[derive(Error, Debug)]
pub enum MdfError {
    /// Error opening a file
    #[error("Failed to open file: {0}")]
    FileOpen(String),

    /// Error reading the header
    #[error("Failed to read header")]
    HeaderRead,

    /// Error reading measurement info
    #[error("Failed to read measurement info")]
    MeasurementInfo,

    /// Error reading data
    #[error("Failed to read data")]
    DataRead,

    /// Error initializing a measurement
    #[error("Failed to initialize measurement")]
    MeasurementInit,

    /// Error finalizing a measurement
    #[error("Failed to finalize measurement")]
    MeasurementFinalize,

    /// Invalid file format
    #[error("Invalid file format")]
    InvalidFormat,

    /// Null pointer encountered
    #[error("Null pointer encountered")]
    NullPointer,

    /// Index out of bounds
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),

    /// Invalid channel type
    #[error("Invalid channel type: {0}")]
    InvalidChannelType(u8),

    /// Invalid data type
    #[error("Invalid data type: {0}")]
    InvalidDataType(u8),

    /// Buffer too small
    #[error("Buffer too small: needed {needed}, got {actual}")]
    BufferTooSmall { needed: usize, actual: usize },

    /// UTF-8 conversion error
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    /// String conversion error
    #[error("String conversion error: {0}")]
    StringConversion(#[from] std::ffi::NulError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// CString conversion error
    #[error("CString conversion error: {0}")]
    CStringConversion(#[from] std::ffi::IntoStringError),

    /// Callback error
    #[error("Callback error: {0}")]
    CallbackError(String),
}

/// Result type for mdflib operations
pub type Result<T> = std::result::Result<T, MdfError>;
