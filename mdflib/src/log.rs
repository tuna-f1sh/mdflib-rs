//! Logging functionality for the mdflib crate.
//!
//! This module provides a safe interface to the logging capabilities of the
//! underlying `mdflib` C++ library. It allows users to set a custom logging
use crate::error::{MdfError, Result};
use mdflib_sys as ffi;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;

/// Re-export of the MdfLogSeverity enum for use in the logging callback.
pub use ffi::MdfLogSeverity;

/// Type alias for the logging callback function.
pub type LogCallback1 = extern "C" fn(severity: MdfLogSeverity, text: *const u8);
pub type LogCallback2 =
    extern "C" fn(severity: MdfLogSeverity, function: *const u8, text: *const u8);

/// A static variable to hold the user-defined logging callback.
///
static LOG_CALLBACK_1: Mutex<Option<LogCallback1>> = Mutex::new(None);
static LOG_CALLBACK_2: Mutex<Option<LogCallback2>> = Mutex::new(None);

/// The C-compatible callback function that will be passed to the C++ library.
extern "C" fn log_callback_wrapper_1(severity: MdfLogSeverity, text: *const c_char) {
    unsafe {
        if let Some(callback) = LOG_CALLBACK_1.lock().unwrap().as_ref() {
            let rust_text = CStr::from_ptr(text).to_string_lossy();
            let bytes = rust_text.as_bytes();
            callback(severity, bytes.as_ptr());
        }
    }
}

extern "C" fn log_callback_wrapper_2(
    severity: MdfLogSeverity,
    function: *const c_char,
    text: *const c_char,
) {
    unsafe {
        if let Some(callback) = LOG_CALLBACK_2.lock().unwrap().as_ref() {
            let rust_function = CStr::from_ptr(function).to_string_lossy();
            let rust_text = CStr::from_ptr(text).to_string_lossy();
            let function_bytes = rust_function.as_bytes();
            let text_bytes = rust_text.as_bytes();
            callback(severity, function_bytes.as_ptr(), text_bytes.as_ptr());
        }
    }
}

/// Sets a custom logging function.
///
/// # Example
///
/// ```
/// use mdflib::log::{set_log_callback_1, MdfLogSeverity};
/// use std::ffi::CStr;
/// use std::os::raw::c_char;
///
/// extern "C" fn my_log_callback(severity: MdfLogSeverity, text: *const u8) {
/// let text = unsafe { CStr::from_ptr(text as *const c_char).to_string_lossy() };
///     println!("[{:?}] {}", severity, text);
/// }
///
/// set_log_callback_1(Some(my_log_callback)).unwrap();
/// ```
pub fn set_log_callback_1(callback: Option<LogCallback1>) -> Result<()> {
    unsafe {
        if let Some(callback) = callback {
            if LOG_CALLBACK_1.lock().unwrap().is_some() {
                return Err(MdfError::CallbackError(
                    "Failed to set log callback, already set".to_string(),
                ));
            }
            LOG_CALLBACK_1.lock().unwrap().replace(callback);
            ffi::MdfSetLogFunction1(Some(log_callback_wrapper_1));
        } else {
            LOG_CALLBACK_1.lock().unwrap().take();
            ffi::MdfSetLogFunction1(None);
        }
    }

    Ok(())
}

pub fn set_log_callback_2(callback: Option<LogCallback2>) -> Result<()> {
    unsafe {
        if let Some(callback) = callback {
            if LOG_CALLBACK_2.lock().unwrap().is_some() {
                return Err(MdfError::CallbackError(
                    "Failed to set log callback, already set".to_string(),
                ));
            }
            LOG_CALLBACK_2.lock().unwrap().replace(callback);
            ffi::MdfSetLogFunction2(Some(log_callback_wrapper_2));
        } else {
            LOG_CALLBACK_2.lock().unwrap().take();
            ffi::MdfSetLogFunction2(None);
        }
    }

    Ok(())
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

/// A C-compatible logging callback function that logs messages with the function name.
pub extern "C" fn log_callback_with_function(
    severity: MdfLogSeverity,
    function: *const u8,
    text: *const u8,
) {
    let function = unsafe { CStr::from_ptr(function as *const c_char).to_string_lossy() };
    let text = unsafe { CStr::from_ptr(text as *const c_char).to_string_lossy() };
    match severity {
        MdfLogSeverity::kTrace => log::trace!("[{function}][{severity:?}]: {text}"),
        MdfLogSeverity::kDebug => log::debug!("[{function}][{severity:?}]: {text}"),
        MdfLogSeverity::kInfo | MdfLogSeverity::kNotice => {
            log::info!("[{function}][{severity:?}]: {text}")
        }
        _ => log::warn!("[{function}][{severity:?}]: {text}"),
    }
}
