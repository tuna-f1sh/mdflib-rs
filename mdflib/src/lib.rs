//! # mdflib
//!
//! Safe Rust bindings for the `mdflib` C++ library. `mdflib` is a library for reading and writing MDF (Measurement Data Format) files.
//! This crate provides a high-level API that wraps the C++ library in safe Rust code.
//!
//! ## Features
//!
//! *   Read and write MDF files (versions 3.x and 4.x).
//! *   Access to file metadata and channel information.
//! *   Read and write channel data.
//! *   `bundled` (default): Compiles and statically links the `mdflib` C++ library.
//! *   `system`: Links against a system-installed version of `mdflib`.
//!
//! See [`crate::MdfReader`] and [`crate::MdfWriter`] docs for examples of how
//! to use the library. The 'examples/read_mdf.rs' and workspace binary
//! 'mf4_candump' provide additional usage examples.

pub mod canmessage;
pub mod channel;
pub mod channelgroup;
pub mod datagroup;
pub mod error;
pub mod file;
pub mod header;
pub mod reader;
pub mod writer;

pub mod log;

// New MDF object modules
pub mod attachment;
pub mod channelarray;
pub mod channelconversion;
pub mod channelobserver;
pub mod etag;
pub mod event;
pub mod filehistory;
pub mod metadata;
pub mod sourceinformation;

pub use canmessage::{CanMessage, CanMessageRef};
pub use channel::{Channel, ChannelRef};
pub use channelgroup::{ChannelGroup, ChannelGroupRef};
pub use datagroup::{DataGroup, DataGroupRef};
pub use error::{MdfError, Result};
pub use file::{MdfFile, MdfFileRef};
pub use header::{MdfHeader, MdfHeaderRef};
pub use reader::MdfReader;
pub use writer::{MdfWriter, MdfWriterType};

// Re-export new MDF object types
pub use attachment::{Attachment, AttachmentRef};
pub use channelarray::{ChannelArray, ChannelArrayRef};
pub use channelconversion::{ChannelConversion, ChannelConversionRef};
pub use channelobserver::{create_channel_observer, ChannelObserver, ChannelObserverRef};
pub use etag::{ETag, ETagRef};
pub use event::{Event, EventRef};
pub use filehistory::{FileHistory, FileHistoryRef};
pub use log::{log_callback, set_log_callback_1};
pub use metadata::{MetaData, MetaDataRef};
pub use sourceinformation::{SourceInformation, SourceInformationRef};
