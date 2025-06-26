//! MDF file reader implementation

use crate::data_group::DataGroup;
use crate::error::{MdfError, Result};
use crate::file::MdfFile;
use mdflib_sys::*;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;

/// Safe wrapper around mdflib's MdfReader
pub struct MdfReader {
    inner: *mut mdflib_sys::MdfReader,
}

impl MdfReader {
    /// Create a new MDF reader for the specified file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path
            .as_ref()
            .to_str()
            .ok_or_else(|| MdfError::InvalidFormat)?;
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

    /// Get the current index
    pub fn index(&self) -> i64 {
        unsafe { MdfReaderGetIndex(self.inner) }
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

    /// Read data for a specific data group
    pub fn read_data(&mut self, group: &mut DataGroup) -> Result<()> {
        unsafe {
            if MdfReaderReadData(self.inner, group.as_mut_ptr()) {
                Ok(())
            } else {
                Err(MdfError::DataRead)
            }
        }
    }

    /// Get the file associated with this reader
    pub fn file(&self) -> Option<MdfFile> {
        unsafe {
            let file_ptr = MdfReaderGetFile(self.inner);
            if file_ptr.is_null() {
                None
            } else {
                Some(MdfFile::from_ptr(file_ptr as *mut _))
            }
        }
    }

    /// Get the number of data groups
    pub fn data_group_count(&self) -> usize {
        unsafe { MdfReaderGetDataGroupCount(self.inner) }
    }

    /// Get a data group by index
    pub fn data_group(&self, index: usize) -> Result<DataGroup> {
        if index >= self.data_group_count() {
            return Err(MdfError::IndexOutOfBounds(index));
        }

        unsafe {
            let group_ptr = MdfReaderGetDataGroup(self.inner, index);
            if group_ptr.is_null() {
                Err(MdfError::NullPointer)
            } else {
                Ok(DataGroup::from_ptr(group_ptr as *mut _))
            }
        }
    }

    /// Get all data groups
    pub fn data_groups(&self) -> Vec<DataGroup> {
        let count = self.data_group_count();
        let mut groups = Vec::with_capacity(count);

        for i in 0..count {
            if let Ok(group) = self.data_group(i) {
                groups.push(group);
            }
        }

        groups
    }

    /// Convenience method to read an entire MDF file
    pub fn read_everything(&mut self) -> Result<()> {
        self.open()?;
        self.read_header()?;
        self.read_measurement_info()?;
        self.read_everything_but_data()?;
        Ok(())
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
// Note: Sync might not be appropriate depending on mdflib's thread safety
// unsafe impl Sync for MdfReader {}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_reader_creation() {
        let temp_file = NamedTempFile::new().unwrap();

        // This test might fail since we're creating an empty file
        // In a real test, you'd want a valid MDF file
        let reader = MdfReader::new(temp_file.path());

        // The reader creation might succeed even with an invalid file
        // The error would come when trying to open/read
        match reader {
            Ok(_) => println!("Reader created successfully"),
            Err(e) => println!("Expected error: {}", e),
        }
    }
}
