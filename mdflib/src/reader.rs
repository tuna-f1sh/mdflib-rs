//! MDF file reader implementation
//!
//! This module provides a safe wrapper around the `mdflib` C++ library's `MdfReader`.
//! It allows for reading MDF files, accessing their headers, data groups, and channel information.
//!
//! It's probably helpful to read the [mdflib reader documentation](https://ihedvall.github.io/mdflib/mdfreader.html) for more details on how to use the reader.
//!
//! See 'examples/read_mdf.rs' for a complete example of how to use this reader.
//!
//! # Example
//!
//! ```no_run
//! use mdflib::{MdfReader, Result};
//!
//! fn main() -> Result<()> {
//!     // Create a new reader for an MDF file.
//!     let mut reader = MdfReader::new("tests/test.mdf")?;
//!
//!     // Read all metadata from the file.
//!     reader.read_everything_but_data()?;
//!
//!     if let Some(header) = reader.get_header() {
//!         println!("File header: {}", header);
//!     }
//!
//!     // Iterate through the data groups and print channel information.
//!     for i in 0..reader.get_data_group_count() {
//!         if let Some(dg) = reader.get_data_group(i) {
//!             for j in 0..dg.get_channel_group_count() {
//!                 if let Some(cg) = dg.get_channel_group_by_index(j) {
//!                     println!("Data Group {}: {} channels", i, cg.get_channel_count());
//!                     for k in 0..cg.get_channel_count() {
//!                         if let Some(cn) = cg.get_channel_by_index(k) {
//!                             println!("  - Channel {}: {}", k, cn.get_name());
//!                         }
//!                     }
//!                 }
//!             }
//!         }
//!     }
//!
//!     reader.close();
//!
//!     Ok(())
//! }
//! ```
use crate::{
    datagroup::{DataGroup, DataGroupRef},
    error::{MdfError, Result},
    header::MdfHeaderRef,
    MdfFileRef,
};
use mdflib_sys::*;
use std::ffi::CString;
use std::path::Path;

/// Safe wrapper around mdflib's MdfReader
pub struct MdfReader {
    inner: *mut mdflib_sys::MdfReader,
}

impl MdfReader {
    /// Create a new MDF reader for the specified file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_str().unwrap();
        let c_path = CString::new(path_str)?;

        unsafe {
            let reader = MdfReaderInit(c_path.as_ptr());
            if reader.is_null() {
                return Err(MdfError::FileOpen(path_str.to_string()));
            }

            Ok(MdfReader { inner: reader })
        }
    }

    /// Check if the reader is in a valid state
    pub fn is_ok(&self) -> bool {
        unsafe { MdfReaderIsOk(self.inner) }
    }

    /// Check if the reader is finialized
    pub fn is_finalized(&self) -> bool {
        unsafe { MdfReaderIsFinalized(self.inner) }
    }

    /// Open the MDF file for reading
    pub fn open(&mut self) -> Result<()> {
        unsafe {
            if MdfReaderOpen(self.inner) {
                Ok(())
            } else {
                Err(MdfError::FileOpen("Failed to open file".to_string()))
            }
        }
    }

    /// Close the MDF file
    pub fn close(&mut self) {
        unsafe {
            MdfReaderClose(self.inner);
        }
    }

    /// Gets the file object.
    pub fn get_file(&self) -> Option<MdfFileRef> {
        unsafe {
            let file = MdfReaderGetFile(self.inner);
            if file.is_null() {
                None
            } else {
                Some(MdfFileRef::new(file))
            }
        }
    }

    /// Read the file header
    pub fn read_header(&mut self) -> Result<()> {
        unsafe {
            if MdfReaderReadHeader(self.inner) {
                Ok(())
            } else {
                Err(MdfError::HeaderRead)
            }
        }
    }

    /// Read measurement information
    pub fn read_measurement_info(&mut self) -> Result<()> {
        unsafe {
            if MdfReaderReadMeasurementInfo(self.inner) {
                Ok(())
            } else {
                Err(MdfError::MeasurementInfo)
            }
        }
    }

    /// Gets the header from the file.
    pub fn get_header(&self) -> Option<MdfHeaderRef> {
        unsafe {
            let header = MdfReaderGetHeader(self.inner);
            if header.is_null() {
                None
            } else {
                Some(MdfHeaderRef::new(header))
            }
        }
    }

    /// Read everything except data
    pub fn read_everything_but_data(&mut self) -> Result<()> {
        unsafe {
            if MdfReaderReadEverythingButData(self.inner) {
                Ok(())
            } else {
                Err(MdfError::DataRead)
            }
        }
    }

    /// Gets the number of data groups in the file.
    pub fn get_data_group_count(&self) -> usize {
        unsafe { MdfReaderGetDataGroupCount(self.inner) }
    }

    /// Gets a data group by its index.
    pub fn get_data_group(&self, index: usize) -> Option<DataGroupRef> {
        unsafe {
            let dg = MdfReaderGetDataGroup(self.inner, index);
            if dg.is_null() {
                None
            } else {
                Some(DataGroupRef::new(dg))
            }
        }
    }

    /// Read data from a data group
    pub fn read_data(&mut self, group: &mut DataGroup) -> Result<()> {
        unsafe {
            if MdfReaderReadData(self.inner, group.inner) {
                Ok(())
            } else {
                Err(MdfError::DataRead)
            }
        }
    }
}

impl Drop for MdfReader {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                MdfReaderUnInit(self.inner);
            }
        }
    }
}

// Ensure MdfReader is Send and Sync if the underlying C++ library supports it
unsafe impl Send for MdfReader {}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    #[ignore] // Ignore this test as it requires a valid MDF file
    fn test_reader_creation() {
        let temp_file = NamedTempFile::new().unwrap();

        // This test might fail since we're creating an empty file
        // In a real test, you'd want a valid MDF file
        let reader = MdfReader::new(temp_file.path());

        // The reader creation might succeed even with an invalid file
        // The error would come when trying to open/read
        match reader {
            Ok(_) => println!("Reader created successfully"),
            Err(e) => println!("Expected error: {e}"),
        }
    }
}
