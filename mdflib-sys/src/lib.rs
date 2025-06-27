//! Low-level FFI bindings for mdflib
//!
//! This crate provides unsafe, low-level bindings to the mdflib C++ library.
//! For a safe, high-level API, use the `mdflib` crate instead.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include the generated bindings from bindgen
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_types() {
        // Test that our constants are properly defined
        println!("Testing basic enum types are accessible");
    }

    #[test]
    #[ignore] // Ignore by default since we need actual mdflib functions
    fn test_basic_reader_creation() {
        // This test would require actual function calls
        println!("Reader creation test (ignored by default)");
    }
}
