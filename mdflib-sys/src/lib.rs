//! Low-level FFI bindings for mdflib
//!
//! This crate provides unsafe, low-level bindings to the mdflib C++ library.
//! For a safe, high-level API, use the `mdflib` crate instead.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include the generated bindings (either from bindgen or fallback)
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_basic_types() {
        // Test that our constants are properly defined
        let _writer_type = MDF_WRITER_TYPE_MDF4;
        let _channel_type = CHANNEL_TYPE_MASTER;
        let _data_type = CHANNEL_DATA_TYPE_FLOAT;
    }

    #[test]
    #[ignore] // Ignore by default since we don't have a real library to test against yet
    fn test_basic_reader_creation() {
        unsafe {
            let filename = CString::new("test.mdf").unwrap();
            let reader = MdfReaderInit(filename.as_ptr());
            if !reader.is_null() {
                MdfReaderUnInit(reader);
            }
        }
    }
}
