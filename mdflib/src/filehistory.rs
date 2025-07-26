//! FileHistory wrapper for mdflib
//!
//! This module provides safe Rust wrappers around the mdflib IFileHistory functionality.

use crate::error::Result;
use crate::metadata::MetaDataRef;
use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

/// Represents an immutable reference to file history in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct FileHistoryRef<'a> {
    pub(crate) inner: *const ffi::IFileHistory,
    _marker: PhantomData<&'a ()>,
}

impl<'a> FileHistoryRef<'a> {
    pub(crate) fn new(inner: *const ffi::IFileHistory) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the index of the file history.
    pub fn get_index(&self) -> u64 {
        unsafe { ffi::FileHistoryGetIndex(self.inner) }
    }

    /// Gets the time of the file history.
    pub fn get_time(&self) -> u64 {
        unsafe { ffi::FileHistoryGetTime(self.inner) }
    }

    /// Gets the description of the file history.
    pub fn get_description(&self) -> String {
        unsafe {
            let mut len = ffi::FileHistoryGetDescription(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::FileHistoryGetDescription(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the tool name of the file history.
    pub fn get_tool_name(&self) -> String {
        unsafe {
            let mut len = ffi::FileHistoryGetToolName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::FileHistoryGetToolName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the tool vendor of the file history.
    pub fn get_tool_vendor(&self) -> String {
        unsafe {
            let mut len = ffi::FileHistoryGetToolVendor(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::FileHistoryGetToolVendor(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the tool version of the file history.
    pub fn get_tool_version(&self) -> String {
        unsafe {
            let mut len = ffi::FileHistoryGetToolVersion(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::FileHistoryGetToolVersion(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the user name of the file history.
    pub fn get_user_name(&self) -> String {
        unsafe {
            let mut len = ffi::FileHistoryGetUserName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::FileHistoryGetUserName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the metadata of the file history.
    pub fn get_metadata(&self) -> Option<MetaDataRef<'a>> {
        unsafe {
            let metadata = ffi::FileHistoryGetMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaDataRef::new(metadata))
            }
        }
    }
}

/// Represents mutable file history in an MDF file.
#[derive(Debug)]
pub struct FileHistory<'a> {
    pub(crate) inner: *mut ffi::IFileHistory,
    inner_ref: FileHistoryRef<'a>,
}

impl<'a> FileHistory<'a> {
    pub(crate) fn new(inner: *mut ffi::IFileHistory) -> Self {
        Self {
            inner,
            inner_ref: FileHistoryRef::new(inner),
        }
    }

    /// Sets the time of the file history.
    pub fn set_time(&mut self, time: u64) {
        unsafe {
            ffi::FileHistorySetTime(self.inner, time);
        }
    }

    /// Sets the description of the file history.
    pub fn set_description(&mut self, description: &str) -> Result<()> {
        let c_description = CString::new(description)?;
        unsafe {
            ffi::FileHistorySetDescription(self.inner, c_description.as_ptr());
        }
        Ok(())
    }

    /// Sets the tool name of the file history.
    pub fn set_tool_name(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name)?;
        unsafe {
            ffi::FileHistorySetToolName(self.inner, c_name.as_ptr());
        }
        Ok(())
    }

    /// Sets the tool vendor of the file history.
    pub fn set_tool_vendor(&mut self, vendor: &str) -> Result<()> {
        let c_vendor = CString::new(vendor)?;
        unsafe {
            ffi::FileHistorySetToolVendor(self.inner, c_vendor.as_ptr());
        }
        Ok(())
    }

    /// Sets the tool version of the file history.
    pub fn set_tool_version(&mut self, version: &str) -> Result<()> {
        let c_version = CString::new(version)?;
        unsafe {
            ffi::FileHistorySetToolVersion(self.inner, c_version.as_ptr());
        }
        Ok(())
    }

    /// Sets the user name of the file history.
    pub fn set_user_name(&mut self, user: &str) -> Result<()> {
        let c_user = CString::new(user)?;
        unsafe {
            ffi::FileHistorySetUserName(self.inner, c_user.as_ptr());
        }
        Ok(())
    }
}

impl<'a> Deref for FileHistory<'a> {
    type Target = FileHistoryRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
