//! ChannelConversion wrapper for mdflib
//!
//! This module provides safe Rust wrappers around the mdflib IChannelConversion functionality.

use crate::error::Result;
use crate::metadata::{MetaData, MetaDataRef};
use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

/// Represents an immutable reference to a channel conversion in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct ChannelConversionRef<'a> {
    pub(crate) inner: *const ffi::IChannelConversion,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ChannelConversionRef<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *const ffi::IChannelConversion) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the index of the channel conversion.
    pub fn get_index(&self) -> u64 {
        unsafe { ffi::ChannelConversionGetIndex(self.inner) }
    }

    /// Gets the name of the channel conversion.
    pub fn get_name(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelConversionGetName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelConversionGetName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the description of the channel conversion.
    pub fn get_description(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelConversionGetDescription(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelConversionGetDescription(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the unit of the channel conversion.
    pub fn get_unit(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelConversionGetUnit(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelConversionGetUnit(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the type of the channel conversion.
    pub fn get_type(&self) -> u8 {
        unsafe { ffi::ChannelConversionGetType(self.inner) }
    }

    /// Gets whether precision is used.
    pub fn is_precision_used(&self) -> bool {
        unsafe { ffi::ChannelConversionIsPrecisionUsed(self.inner) }
    }

    /// Gets the precision.
    pub fn get_precision(&self) -> u8 {
        unsafe { ffi::ChannelConversionGetPrecision(self.inner) }
    }

    /// Gets whether range is used.
    pub fn is_range_used(&self) -> bool {
        unsafe { ffi::ChannelConversionIsRangeUsed(self.inner) }
    }

    /// Gets the range minimum.
    pub fn get_range_min(&self) -> f64 {
        unsafe { ffi::ChannelConversionGetRangeMin(self.inner) }
    }

    /// Gets the range maximum.
    pub fn get_range_max(&self) -> f64 {
        unsafe { ffi::ChannelConversionGetRangeMax(self.inner) }
    }

    /// Gets the flags.
    pub fn get_flags(&self) -> u16 {
        unsafe { ffi::ChannelConversionGetFlags(self.inner) }
    }

    /// Gets the formula.
    pub fn get_formula(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelConversionGetFormula(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelConversionGetFormula(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets a parameter as a double.
    pub fn get_parameter_as_double(&self, index: u16) -> f64 {
        unsafe { ffi::ChannelConversionGetParameterAsDouble(self.inner, index) }
    }

    /// Gets a parameter as a uint64.
    pub fn get_parameter_as_uint64(&self, index: u16) -> u64 {
        unsafe { ffi::ChannelConversionGetParameterAsUInt64(self.inner, index) }
    }

    /// Gets the metadata.
    pub fn get_metadata(&self) -> Option<MetaDataRef<'a>> {
        unsafe {
            let metadata = ffi::ChannelConversionGetMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaDataRef::new(metadata))
            }
        }
    }
}

/// Represents a mutable channel conversion in an MDF file.
#[derive(Debug)]
pub struct ChannelConversion<'a> {
    pub(crate) inner: *mut ffi::IChannelConversion,
    inner_ref: ChannelConversionRef<'a>,
}

impl<'a> ChannelConversion<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *mut ffi::IChannelConversion) -> Self {
        Self {
            inner,
            inner_ref: ChannelConversionRef::new(inner),
        }
    }

    /// Sets the name of the channel conversion.
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name)?;
        unsafe {
            ffi::ChannelConversionSetName(self.inner, c_name.as_ptr());
        }
        Ok(())
    }

    /// Sets the description of the channel conversion.
    pub fn set_description(&mut self, description: &str) -> Result<()> {
        let c_description = CString::new(description)?;
        unsafe {
            ffi::ChannelConversionSetDescription(self.inner, c_description.as_ptr());
        }
        Ok(())
    }

    /// Sets the unit of the channel conversion.
    pub fn set_unit(&mut self, unit: &str) -> Result<()> {
        let c_unit = CString::new(unit)?;
        unsafe {
            ffi::ChannelConversionSetUnit(self.inner, c_unit.as_ptr());
        }
        Ok(())
    }

    /// Sets the type of the channel conversion.
    pub fn set_type(&mut self, conversion_type: u8) {
        unsafe {
            ffi::ChannelConversionSetType(self.inner, conversion_type);
        }
    }

    /// Sets the range.
    pub fn set_range(&mut self, min: f64, max: f64) {
        unsafe {
            ffi::ChannelConversionSetRange(self.inner, min, max);
        }
    }

    /// Sets the formula.
    pub fn set_formula(&mut self, formula: &str) -> Result<()> {
        let c_formula = CString::new(formula)?;
        unsafe {
            ffi::ChannelConversionSetFormula(self.inner, c_formula.as_ptr());
        }
        Ok(())
    }

    /// Sets a parameter as a double.
    pub fn set_parameter_as_double(&mut self, index: u16, parameter: f64) {
        unsafe {
            ffi::ChannelConversionSetParameterAsDouble(self.inner, index, parameter);
        }
    }

    /// Sets a parameter as a uint64.
    pub fn set_parameter_as_uint64(&mut self, index: u16, parameter: u64) {
        unsafe {
            ffi::ChannelConversionSetParameterAsUInt64(self.inner, index, parameter);
        }
    }

    /// Creates metadata for the channel conversion.
    pub fn create_metadata(&mut self) -> Option<MetaData<'a>> {
        unsafe {
            let metadata = ffi::ChannelConversionCreateMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaData::new(metadata))
            }
        }
    }
}

impl<'a> Deref for ChannelConversion<'a> {
    type Target = ChannelConversionRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
