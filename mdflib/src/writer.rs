//! MDF file writer implementation provides a safe wrapper around the `mdflib`
//! C++ library's `MdfWriter`. It allows for creating and writing data to MDF
//! files.
//!
//! It's probably helpful to read the [mdflib writer documentation](https://ihedvall.github.io/mdflib/mdfwriter.html) for more details on how to use the writer.
//!
//! # Example
//!
//! ```no_run
//! use mdflib::{MdfWriter, MdfWriterType, Result};
//! use std::time::{SystemTime, UNIX_EPOCH};
//!
//! fn main() -> Result<()> {
//!     // Create a new writer for an MDF file.
//!     let mut writer = MdfWriter::new(MdfWriterType::Mdf4Basic, "test.mdf")?;
//!
//!     // Create a new data group.
//!     if let Some(mut dg) = writer.create_data_group() {
//!         // Create a new channel group.
//!         if let Some(mut cg) = dg.create_channel_group() {
//!             // Create a new channel.
//!             if let Some(mut cn) = cg.create_channel() {
//!                 cn.set_unit("s");
//!             }
//!         }
//!     }
//!
//!     // Initialize the measurement.
//!     writer.init_measurement();
//!
//!     // Start the measurement.
//!     let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
//!     writer.start_measurement(start_time);
//!
//!     // ... write some data ...
//!
//!     // Stop the measurement.
//!     let stop_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
//!     writer.stop_measurement(stop_time);
//!
//!     // Finalize the measurement.
//!     writer.finalize_measurement();
//!
//!     Ok(())
//! }
//! ```
use crate::{
    canmessage::CanMessageRef,
    channelgroup::ChannelGroupRef,
    datagroup::DataGroup,
    error::{MdfError, Result},
    file::MdfFile,
    header::MdfHeader,
};
use mdflib_sys::*;
use std::ffi::CString;
use std::path::Path;

pub use mdflib_sys::MdfWriterType;

/// Safe wrapper around mdflib's MdfWriter
pub struct MdfWriter {
    inner: *mut mdflib_sys::MdfWriter,
}

impl MdfWriter {
    /// Create a new MDF writer for the specified file
    pub fn new<P: AsRef<Path>>(writer_type: MdfWriterType, path: P) -> Result<Self> {
        let path_str = path.as_ref().to_str().unwrap();
        let c_path = CString::new(path_str)?;

        unsafe {
            let writer = MdfWriterInit(writer_type, c_path.as_ptr());
            if writer.is_null() {
                return Err(MdfError::FileOpen(path_str.to_string()));
            }

            Ok(MdfWriter { inner: writer })
        }
    }

    /// Gets the file object from the writer.
    pub fn get_file(&self) -> Option<MdfFile> {
        unsafe {
            let file = MdfWriterGetFile(self.inner);
            if file.is_null() {
                None
            } else {
                Some(MdfFile::new(file))
            }
        }
    }

    /// Gets the header from the file.
    pub fn get_header(&self) -> Option<MdfHeader> {
        unsafe {
            let header = MdfWriterGetHeader(self.inner);
            if header.is_null() {
                None
            } else {
                Some(MdfHeader::new(header))
            }
        }
    }

    /// Check if the file is new
    pub fn is_file_new(&self) -> bool {
        unsafe { MdfWriterIsFileNew(self.inner) }
    }

    /// Get compress data flag
    pub fn get_compress_data(&self) -> bool {
        unsafe { MdfWriterGetCompressData(self.inner) }
    }

    /// Set compress data flag
    pub fn set_compress_data(&mut self, compress: bool) {
        unsafe { MdfWriterSetCompressData(self.inner, compress) }
    }

    /// Get pre-trigger time
    pub fn get_pre_trig_time(&self) -> f64 {
        unsafe { MdfWriterGetPreTrigTime(self.inner) }
    }

    /// Set pre-trigger time
    pub fn set_pre_trig_time(&mut self, pre_trig_time: f64) {
        unsafe { MdfWriterSetPreTrigTime(self.inner, pre_trig_time) }
    }

    /// Get start time
    pub fn get_start_time(&self) -> u64 {
        unsafe { MdfWriterGetStartTime(self.inner) }
    }

    /// Get stop time
    pub fn get_stop_time(&self) -> u64 {
        unsafe { MdfWriterGetStopTime(self.inner) }
    }

    /// Get bus type
    pub fn get_bus_type(&self) -> u16 {
        unsafe { MdfWriterGetBusType(self.inner) }
    }

    /// Set bus type
    pub fn set_bus_type(&mut self, bus_type: u16) {
        unsafe { MdfWriterSetBusType(self.inner, bus_type) }
    }

    /// Create bus log configuration
    pub fn create_bus_log_configuration(&mut self) -> bool {
        unsafe { MdfWriterCreateBusLogConfiguration(self.inner) }
    }

    /// Create a new data group
    pub fn create_data_group(&mut self) -> Option<DataGroup> {
        unsafe {
            let dg = MdfWriterCreateDataGroup(self.inner);
            if dg.is_null() {
                None
            } else {
                Some(DataGroup::new(dg))
            }
        }
    }

    /// Initialize measurement
    pub fn init_measurement(&mut self) -> bool {
        unsafe { MdfWriterInitMeasurement(self.inner) }
    }

    /// Save a sample
    ///
    /// Time is absolute time in nanoseconds since the epoch (1970-01-01T00:00:00Z).
    pub fn save_sample(&mut self, group: &ChannelGroupRef, time: u64) {
        unsafe { MdfWriterSaveSample(self.inner, group.inner, time) }
    }

    /// Save a CAN message
    ///
    /// Time is absolute time in nanoseconds since the epoch (1970-01-01T00:00:00Z).
    pub fn save_can_message(
        &mut self,
        group: &ChannelGroupRef,
        time: u64,
        message: &CanMessageRef,
    ) {
        unsafe { MdfWriterSaveCanMessage(self.inner, group.inner, time, message.inner) }
    }

    /// Start measurement
    ///
    /// Time is absolute time in nanoseconds since the epoch (1970-01-01T00:00:00Z). **Should be > 0 otherwise samples will not be saved.**
    pub fn start_measurement(&mut self, start_time: u64) {
        unsafe { MdfWriterStartMeasurement(self.inner, start_time) }
    }

    /// Stop measurement
    ///
    /// Time is absolute time in nanoseconds since the epoch (1970-01-01T00:00:00Z). Should be greater than or equal to the start time.
    pub fn stop_measurement(&mut self, stop_time: u64) {
        unsafe { MdfWriterStopMeasurement(self.inner, stop_time) }
    }

    /// Finalize measurement
    ///
    /// Unloads worker queue, joins threads, and writes the final data to the file.
    pub fn finalize_measurement(&mut self) -> bool {
        unsafe { MdfWriterFinalizeMeasurement(self.inner) }
    }
}

impl Drop for MdfWriter {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                MdfWriterUnInit(self.inner);
            }
        }
    }
}

// Ensure MdfWriter is Send and Sync if the underlying C++ library supports it
unsafe impl Send for MdfWriter {}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_writer_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let writer = MdfWriter::new(MdfWriterType::Mdf4Basic, temp_file.path());
        assert!(writer.is_ok());
    }
}
