//! MDF file writer implementation
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

    /// Gets a channel group by name from the last data group.
    ///
    /// # Safety
    /// The returned ChannelGroupRef is only valid as long as this MdfWriter exists.
    /// Do not use the returned reference after the writer has been dropped.
    pub fn get_channel_group(&self, name: &str) -> Option<ChannelGroupRef> {
        let header = self.get_header()?;
        let last_dg = header.get_last_data_group()?;
        last_dg.get_channel_group_ref(name)
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
    pub fn save_sample(&mut self, group: &ChannelGroupRef, time: u64) {
        unsafe { MdfWriterSaveSample(self.inner, group.inner, time) }
    }

    /// Save a CAN message
    pub fn save_can_message(
        &mut self,
        group: &ChannelGroupRef,
        time: u64,
        message: &CanMessageRef,
    ) {
        unsafe { MdfWriterSaveCanMessage(self.inner, group.inner, time, message.inner) }
    }

    /// Start measurement
    pub fn start_measurement(&mut self, start_time: u64) {
        unsafe { MdfWriterStartMeasurement(self.inner, start_time) }
    }

    /// Stop measurement
    pub fn stop_measurement(&mut self, stop_time: u64) {
        unsafe { MdfWriterStopMeasurement(self.inner, stop_time) }
    }

    /// Finalize measurement
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
// Note: Sync might not be appropriate depending on mdflib's thread safety
// unsafe impl Sync for MdfWriter {}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_writer_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let writer = MdfWriter::new(MdfWriterType::MdfWriterType_Mdf4, temp_file.path());
        assert!(writer.is_ok());
    }
}
