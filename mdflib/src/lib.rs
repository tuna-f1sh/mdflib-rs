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
pub mod file;
pub mod header;
pub mod reader;
pub mod writer;

// New MDF object modules
pub mod attachment;
pub mod channelarray;
pub mod channelconversion;
pub mod etag;
pub mod event;
pub mod filehistory;
pub mod metadata;
pub mod sourceinformation;

pub use canmessage::CanMessage;
pub use channel::{Channel, ChannelRef};
pub use channelgroup::{ChannelGroup, ChannelGroupRef};
pub use datagroup::{DataGroup, DataGroupRef};
pub use error::{MdfError, Result};
pub use file::{MdfFile, MdfFileRef};
pub use header::{MdfHeader, MdfHeaderRef};
pub use reader::MdfReader;

// Re-export new MDF object types
pub use attachment::{Attachment, AttachmentRef};
pub use channelarray::{ChannelArray, ChannelArrayRef};
pub use channelconversion::{ChannelConversion, ChannelConversionRef};
pub use etag::{ETag, ETagRef};
pub use event::{Event, EventRef};
pub use filehistory::{FileHistory, FileHistoryRef};
pub use metadata::{MetaData, MetaDataRef};
pub use sourceinformation::{SourceInformation, SourceInformationRef};

#[cfg(test)]
mod tests {}
