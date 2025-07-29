//! Logging functionality for the mdflib crate.
//!
//! This module provides a safe interface to the logging capabilities of the
//! underlying `mdflib` C++ library. It allows users to set a custom logging
use mdflib_sys as ffi;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Re-export of the MdfLogSeverity enum for use in the logging callback.
pub use ffi::MdfLogSeverity;

/// A struct that holds information about the source code location of a log message.
#[derive(Debug, Clone)]
pub struct LogLocation {
    pub line: i32,
    pub column: i32,
    pub file: String,
    pub function: String,
}

impl From<&ffi::MdfLocation> for LogLocation {
    fn from(location: &ffi::MdfLocation) -> Self {
        Self {
            line: location.line,
            column: location.column,
            file: unsafe { CStr::from_ptr(location.file).to_string_lossy().into_owned() },
            function: unsafe {
                CStr::from_ptr(location.function)
                    .to_string_lossy()
                    .into_owned()
            },
        }
    }
}

/// Type alias for the logging callback function.
pub type LogCallback1 = extern "C" fn(severity: MdfLogSeverity, text: *const u8);

/// A static variable to hold the user-defined logging callback.
static mut LOG_CALLBACK_1: Option<LogCallback1> = None;

/// The C-compatible callback function that will be passed to the C++ library.
extern "C" fn log_callback_wrapper_1(
    // location: *const ffi::MdfLocation,
    severity: MdfLogSeverity,
    text: *const c_char,
) {
    unsafe {
        if let Some(callback) = LOG_CALLBACK_1 {
            // let rust_location = LogLocation::from(&*location);
            let rust_text = CStr::from_ptr(text).to_string_lossy();
            let bytes = rust_text.as_bytes();
            callback(severity, bytes.as_ptr());
        }
    }
}

/// Sets a custom logging function.
///
/// # Example
///
/// ```
/// use mdflib::log::{set_log_callback_1, LogLocation, MdfLogSeverity};
///
/// extern "C" fn my_log_callback(location: &LogLocation, severity: MdfLogSeverity, text: &str) {
///     println!("[{:?}] {}:{}: {}", severity, location.file, location.line, text);
/// }
///
/// set_log_callback_1(Some(my_log_callback));
/// ```
pub fn set_log_callback_1(callback: Option<LogCallback1>) {
    unsafe {
        LOG_CALLBACK_1 = callback;
        if callback.is_some() {
            ffi::MdfSetLogFunction1(Some(log_callback_wrapper_1));
        } else {
            ffi::MdfSetLogFunction1(None);
        }
    }
}

/// A C-compatible logging callback function that logs messages using the `log` crate.
pub extern "C" fn log_callback(severity: MdfLogSeverity, text: *const u8) {
    let text = unsafe { CStr::from_ptr(text as *const c_char).to_string_lossy() };
    match severity {
        MdfLogSeverity::kTrace => log::trace!("[{severity:?}]: {text}"),
        MdfLogSeverity::kDebug => log::debug!("[{severity:?}]: {text}"),
        MdfLogSeverity::kInfo | MdfLogSeverity::kNotice => {
            log::info!("[{severity:?}]: {text}")
        }
        _ => log::warn!("[{severity:?}]: {text}"),
    }
}
