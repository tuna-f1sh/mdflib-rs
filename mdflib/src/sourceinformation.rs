//! SourceInformation wrapper for mdflib
//!
//! This module provides safe Rust wrappers around the mdflib ISourceInformation functionality.

use crate::error::Result;
use crate::metadata::{MetaData, MetaDataRef};
use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

/// Represents an immutable reference to source information in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct SourceInformationRef<'a> {
    pub(crate) inner: *const ffi::ISourceInformation,
    _marker: PhantomData<&'a ()>,
}

impl<'a> SourceInformationRef<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *const ffi::ISourceInformation) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the index of the source information.
    pub fn get_index(&self) -> u64 {
        unsafe { ffi::SourceInformationGetIndex(self.inner) }
    }

    /// Gets the name of the source information.
    pub fn get_name(&self) -> String {
        unsafe {
            let mut len = ffi::SourceInformationGetName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::SourceInformationGetName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the description of the source information.
    pub fn get_description(&self) -> String {
        unsafe {
            let mut len = ffi::SourceInformationGetDescription(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::SourceInformationGetDescription(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the path of the source information.
    pub fn get_path(&self) -> String {
        unsafe {
            let mut len = ffi::SourceInformationGetPath(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::SourceInformationGetPath(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the type of the source information.
    pub fn get_type(&self) -> u8 {
        unsafe { ffi::SourceInformationGetType(self.inner) }
    }

    /// Gets the bus type of the source information.
    pub fn get_bus(&self) -> u8 {
        unsafe { ffi::SourceInformationGetBus(self.inner) }
    }

    /// Gets the flags of the source information.
    pub fn get_flags(&self) -> u8 {
        unsafe { ffi::SourceInformationGetFlags(self.inner) }
    }

    /// Gets the metadata of the source information.
    pub fn get_metadata(&self) -> Option<MetaDataRef<'a>> {
        unsafe {
            let metadata = ffi::SourceInformationGetMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaDataRef::new(metadata))
            }
        }
    }
}

/// Represents mutable source information in an MDF file.
#[derive(Debug)]
pub struct SourceInformation<'a> {
    pub(crate) inner: *mut ffi::ISourceInformation,
    inner_ref: SourceInformationRef<'a>,
}

impl<'a> SourceInformation<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *mut ffi::ISourceInformation) -> Self {
        Self {
            inner,
            inner_ref: SourceInformationRef::new(inner),
        }
    }

    /// Sets the name of the source information.
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name)?;
        unsafe {
            ffi::SourceInformationSetName(self.inner, c_name.as_ptr());
        }
        Ok(())
    }

    /// Sets the description of the source information.
    pub fn set_description(&mut self, description: &str) -> Result<()> {
        let c_description = CString::new(description)?;
        unsafe {
            ffi::SourceInformationSetDescription(self.inner, c_description.as_ptr());
        }
        Ok(())
    }

    /// Sets the path of the source information.
    pub fn set_path(&mut self, path: &str) -> Result<()> {
        let c_path = CString::new(path)?;
        unsafe {
            ffi::SourceInformationSetPath(self.inner, c_path.as_ptr());
        }
        Ok(())
    }

    /// Sets the type of the source information.
    pub fn set_type(&mut self, source_type: u8) {
        unsafe {
            ffi::SourceInformationSetType(self.inner, source_type);
        }
    }

    /// Sets the bus type of the source information.
    pub fn set_bus(&mut self, bus: u8) {
        unsafe {
            ffi::SourceInformationSetBus(self.inner, bus);
        }
    }

    /// Sets the flags of the source information.
    pub fn set_flags(&mut self, flags: u8) {
        unsafe {
            ffi::SourceInformationSetFlags(self.inner, flags);
        }
    }

    /// Creates metadata for the source information.
    pub fn create_metadata(&mut self) -> Option<MetaData<'a>> {
        unsafe {
            let metadata = ffi::SourceInformationCreateMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaData::new(metadata))
            }
        }
    }
}

impl<'a> Deref for SourceInformation<'a> {
    type Target = SourceInformationRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
