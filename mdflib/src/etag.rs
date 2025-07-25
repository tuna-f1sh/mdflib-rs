//! ETag wrapper for mdflib
//!
//! This module provides safe Rust wrappers around the mdflib ETag functionality.

use mdflib_sys as ffi;
use crate::error::{MdfError, Result};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

/// Represents an immutable reference to an ETag in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct ETagRef<'a> {
    pub(crate) inner: *const ffi::ETag,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ETagRef<'a> {
    pub(crate) fn new(inner: *const ffi::ETag) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the name of the ETag.
    pub fn get_name(&self) -> String {
        unsafe {
            let mut len = ffi::ETagGetName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ETagGetName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the description of the ETag.
    pub fn get_description(&self) -> String {
        unsafe {
            let mut len = ffi::ETagGetDescription(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ETagGetDescription(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the unit of the ETag.
    pub fn get_unit(&self) -> String {
        unsafe {
            let mut len = ffi::ETagGetUnit(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ETagGetUnit(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the unit reference of the ETag.
    pub fn get_unit_ref(&self) -> String {
        unsafe {
            let mut len = ffi::ETagGetUnitRef(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ETagGetUnitRef(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the type of the ETag.
    pub fn get_type(&self) -> String {
        unsafe {
            let mut len = ffi::ETagGetType(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ETagGetType(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the data type of the ETag.
    pub fn get_data_type(&self) -> u8 {
        unsafe { ffi::ETagGetDataType(self.inner) }
    }

    /// Gets the language of the ETag.
    pub fn get_language(&self) -> String {
        unsafe {
            let mut len = ffi::ETagGetLanguage(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ETagGetLanguage(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets whether the ETag is read only.
    pub fn get_read_only(&self) -> bool {
        unsafe { ffi::ETagGetReadOnly(self.inner) }
    }

    /// Gets the value as a string.
    pub fn get_value_as_string(&self) -> String {
        unsafe {
            let mut len = ffi::ETagGetValueAsString(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ETagGetValueAsString(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the value as a float.
    pub fn get_value_as_float(&self) -> f64 {
        unsafe { ffi::ETagGetValueAsFloat(self.inner) }
    }

    /// Gets the value as a boolean.
    pub fn get_value_as_boolean(&self) -> bool {
        unsafe { ffi::ETagGetValueAsBoolean(self.inner) }
    }

    /// Gets the value as a signed integer.
    pub fn get_value_as_signed(&self) -> i64 {
        unsafe { ffi::ETagGetValueAsSigned(self.inner) }
    }

    /// Gets the value as an unsigned integer.
    pub fn get_value_as_unsigned(&self) -> u64 {
        unsafe { ffi::ETagGetValueAsUnsigned(self.inner) }
    }
}

/// Represents a mutable ETag in an MDF file.
#[derive(Debug)]
pub struct ETag<'a> {
    pub(crate) inner: *mut ffi::ETag,
    inner_ref: ETagRef<'a>,
    owned: bool,
}

impl<'a> ETag<'a> {
    /// Creates a new ETag.
    pub fn new() -> Result<Self> {
        let inner = unsafe { ffi::ETagInit() };
        if inner.is_null() {
            return Err(MdfError::NullPointer);
        }
        Ok(Self {
            inner,
            inner_ref: ETagRef::new(inner),
            owned: true,
        })
    }

    pub(crate) fn from_raw(inner: *mut ffi::ETag) -> Self {
        Self {
            inner,
            inner_ref: ETagRef::new(inner),
            owned: false,
        }
    }

    /// Sets the name of the ETag.
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name)?;
        unsafe {
            ffi::ETagSetName(self.inner, c_name.as_ptr());
        }
        Ok(())
    }

    /// Sets the description of the ETag.
    pub fn set_description(&mut self, description: &str) -> Result<()> {
        let c_description = CString::new(description)?;
        unsafe {
            ffi::ETagSetDescription(self.inner, c_description.as_ptr());
        }
        Ok(())
    }

    /// Sets the unit of the ETag.
    pub fn set_unit(&mut self, unit: &str) -> Result<()> {
        let c_unit = CString::new(unit)?;
        unsafe {
            ffi::ETagSetUnit(self.inner, c_unit.as_ptr());
        }
        Ok(())
    }

    /// Sets the unit reference of the ETag.
    pub fn set_unit_ref(&mut self, unit_ref: &str) -> Result<()> {
        let c_unit_ref = CString::new(unit_ref)?;
        unsafe {
            ffi::ETagSetUnitRef(self.inner, c_unit_ref.as_ptr());
        }
        Ok(())
    }

    /// Sets the type of the ETag.
    pub fn set_type(&mut self, tag_type: &str) -> Result<()> {
        let c_type = CString::new(tag_type)?;
        unsafe {
            ffi::ETagSetType(self.inner, c_type.as_ptr());
        }
        Ok(())
    }

    /// Sets the data type of the ETag.
    pub fn set_data_type(&mut self, data_type: u8) {
        unsafe {
            ffi::ETagSetDataType(self.inner, data_type);
        }
    }

    /// Sets the language of the ETag.
    pub fn set_language(&mut self, language: &str) -> Result<()> {
        let c_language = CString::new(language)?;
        unsafe {
            ffi::ETagSetLanguage(self.inner, c_language.as_ptr());
        }
        Ok(())
    }

    /// Sets whether the ETag is read only.
    pub fn set_read_only(&mut self, read_only: bool) {
        unsafe {
            ffi::ETagSetReadOnly(self.inner, read_only);
        }
    }

    /// Sets the value as a string.
    pub fn set_value_as_string(&mut self, value: &str) -> Result<()> {
        let c_value = CString::new(value)?;
        unsafe {
            ffi::ETagSetValueAsString(self.inner, c_value.as_ptr());
        }
        Ok(())
    }

    /// Sets the value as a float.
    pub fn set_value_as_float(&mut self, value: f64) {
        unsafe {
            ffi::ETagSetValueAsFloat(self.inner, value);
        }
    }

    /// Sets the value as a boolean.
    pub fn set_value_as_boolean(&mut self, value: bool) {
        unsafe {
            ffi::ETagSetValueAsBoolean(self.inner, value);
        }
    }

    /// Sets the value as a signed integer.
    pub fn set_value_as_signed(&mut self, value: i64) {
        unsafe {
            ffi::ETagSetValueAsSigned(self.inner, value);
        }
    }

    /// Sets the value as an unsigned integer.
    pub fn set_value_as_unsigned(&mut self, value: u64) {
        unsafe {
            ffi::ETagSetValueAsUnsigned(self.inner, value);
        }
    }
}

impl<'a> Deref for ETag<'a> {
    type Target = ETagRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}

impl<'a> Drop for ETag<'a> {
    fn drop(&mut self) {
        if self.owned && !self.inner.is_null() {
            unsafe {
                ffi::ETagUnInit(self.inner);
            }
        }
    }
}

impl<'a> Default for ETag<'a> {
    fn default() -> Self {
        Self::new().unwrap()
    }
}