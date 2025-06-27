//! Safe Rust bindings for mdflib
//!
//! This crate provides a safe, high-level interface to the mdflib C++ library
//! for reading and writing MDF (Measurement Data Format) files.
//!
//! # Features
//!
//! - `bundled` (default): Compile and link mdflib from source
//! - `system`: Link against system-installed mdflib
//!
//! # Examples
//!
//! ```no_run
//! use mdflib::{MdfReader, Result};
//!
//! fn read_mdf_file() -> Result<()> {
//!     let mut reader = MdfReader::new("example.mdf").unwrap();
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod reader;
// pub mod writer;
// pub mod channel;
// pub mod data_group;

pub use error::{MdfError, Result};
pub use reader::MdfReader;

#[cfg(test)]
mod tests {}
