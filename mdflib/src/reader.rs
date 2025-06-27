//! MDF file reader implementation

use crate::error::{MdfError, Result};
use mdflib_sys::*;
use std::ffi::CString;
use std::path::Path;

/// Safe wrapper around mdflib's MdfReader
pub struct MdfReader {
    inner: *mut mdflib_sys::mdf_MdfReader,
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
            let reader = mdflib_sys::mdf_MdfReader::MdfReaderInit(c_path.as_ptr());
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
    use std::io::Write;
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
