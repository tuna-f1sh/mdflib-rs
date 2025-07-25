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

pub mod canmessage;
pub mod channel;
pub mod channelgroup;
pub mod datagroup;
pub mod error;
pub mod header;
pub mod reader;
pub mod writer;
pub mod file;

pub use canmessage::CanMessage;
pub use channel::{Channel, ChannelRef};
pub use channelgroup::{ChannelGroup, ChannelGroupRef};
pub use datagroup::{DataGroup, DataGroupRef};
pub use error::{MdfError, Result};
pub use header::{MdfHeader, MdfHeaderRef};
pub use reader::MdfReader;
pub use file::{MdfFile, MdfFileRef};

#[cfg(test)]
mod tests {}
