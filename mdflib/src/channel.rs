//! Channel wrapper for mdflib IChannel
//!
//! A channel represents a single measurement signal in an MDF file. It contains
//! metadata about the signal, such as its name, unit, and data type. It also
//! provides access to the channel's data through a channel observer.

use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

use crate::channelarray::{ChannelArray, ChannelArrayRef};
use crate::channelconversion::{ChannelConversion, ChannelConversionRef};
use crate::metadata::{MetaData, MetaDataRef};
use crate::sourceinformation::{SourceInformation, SourceInformationRef};

/// Represents an immutable reference to a channel in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct ChannelRef<'a> {
    pub(crate) inner: *const ffi::IChannel,
    _marker: PhantomData<&'a ()>,
}

impl std::fmt::Display for ChannelRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // display all meta fields
        write!(
            f,
            "Channel {{ index: {}, name: {}, display_name: {}, description: {}, unit: {}, type: {}, data_type: {}, data_bytes: {:#x} }}",
            self.get_index(),
            self.get_name(),
            self.get_display_name(),
            self.get_description(),
            self.get_unit(),
            self.get_type(),
            self.get_data_type(),
            self.get_data_bytes()
        )
    }
}

impl<'a> ChannelRef<'a> {
    pub(crate) fn new(inner: *const ffi::IChannel) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the raw pointer to the underlying IChannel.
    /// This is used for advanced operations like creating channel observers.
    pub fn as_ptr(&self) -> *const ffi::IChannel {
        self.inner
    }

    /// Gets the index of the channel.
    pub fn get_index(&self) -> u64 {
        unsafe { ffi::ChannelGetIndex(self.inner) }
    }

    /// Gets the name of the channel.
    pub fn get_name(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelGetName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelGetName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the display name of the channel.
    pub fn get_display_name(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelGetDisplayName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelGetDisplayName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the description of the channel.
    pub fn get_description(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelGetDescription(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelGetDescription(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the unit of the channel.
    pub fn get_unit(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelGetUnit(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelGetUnit(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the type of the channel.
    pub fn get_type(&self) -> u8 {
        unsafe { ffi::ChannelGetType(self.inner) }
    }

    /// Gets the data type of the channel.
    pub fn get_data_type(&self) -> u8 {
        unsafe { ffi::ChannelGetDataType(self.inner) }
    }

    /// Gets the data bytes of the channel.
    pub fn get_data_bytes(&self) -> u64 {
        unsafe { ffi::ChannelGetDataBytes(self.inner) }
    }

    /// Gets the metadata of the channel.
    pub fn get_metadata(&self) -> Option<MetaDataRef> {
        unsafe {
            let metadata = ffi::ChannelGetMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaDataRef::new(metadata))
            }
        }
    }

    /// Gets the source information of the channel.
    pub fn get_source_information(&self) -> Option<SourceInformationRef> {
        unsafe {
            let source_info = ffi::ChannelGetSourceInformation(self.inner);
            if source_info.is_null() {
                None
            } else {
                Some(SourceInformationRef::new(source_info))
            }
        }
    }

    /// Gets the channel conversion of the channel.
    pub fn get_channel_conversion(&self) -> Option<ChannelConversionRef> {
        unsafe {
            let conversion = ffi::ChannelGetChannelConversion(self.inner);
            if conversion.is_null() {
                None
            } else {
                Some(ChannelConversionRef::new(conversion))
            }
        }
    }

    /// Gets the channel array of the channel.
    pub fn get_channel_array(&self) -> Option<ChannelArrayRef> {
        unsafe {
            let array = ffi::ChannelGetChannelArray(self.inner);
            if array.is_null() {
                None
            } else {
                Some(ChannelArrayRef::new(array))
            }
        }
    }
}

/// Represents a mutable reference to a channel in an MDF file.
#[derive(Debug)]
pub struct Channel<'a> {
    pub(crate) inner: *mut ffi::IChannel,
    inner_ref: ChannelRef<'a>,
}

impl<'a> Channel<'a> {
    pub(crate) fn new(inner: *mut ffi::IChannel) -> Self {
        Self {
            inner,
            inner_ref: ChannelRef::new(inner),
        }
    }

    /// Sets the name of the channel.
    pub fn set_name(&mut self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            ffi::ChannelSetName(self.inner, c_name.as_ptr());
        }
    }

    /// Sets the display name of the channel.
    pub fn set_display_name(&mut self, display_name: &str) {
        let c_display_name = CString::new(display_name).unwrap();
        unsafe {
            ffi::ChannelSetDisplayName(self.inner, c_display_name.as_ptr());
        }
    }

    /// Sets the description of the channel.
    pub fn set_description(&mut self, description: &str) {
        let c_description = CString::new(description).unwrap();
        unsafe {
            ffi::ChannelSetDescription(self.inner, c_description.as_ptr());
        }
    }

    /// Sets the unit of the channel.
    pub fn set_unit(&mut self, unit: &str) {
        let c_unit = CString::new(unit).unwrap();
        unsafe {
            ffi::ChannelSetUnit(self.inner, c_unit.as_ptr());
        }
    }

    /// Sets the type of the channel.
    pub fn set_type(&mut self, channel_type: u8) {
        unsafe {
            ffi::ChannelSetType(self.inner, channel_type);
        }
    }

    /// Sets the data type of the channel.
    pub fn set_data_type(&mut self, data_type: u8) {
        unsafe {
            ffi::ChannelSetDataType(self.inner, data_type);
        }
    }

    /// Sets the data bytes of the channel.
    pub fn set_data_bytes(&mut self, bytes: u64) {
        unsafe {
            ffi::ChannelSetDataBytes(self.inner, bytes);
        }
    }

    /// Sets the channel value.
    pub fn set_channel_value(&mut self, value: u32, valid: bool) {
        unsafe {
            ffi::ChannelSetChannelValue(self.inner, value, valid);
        }
    }

    /// Creates metadata for the channel.
    pub fn create_metadata(&mut self) -> Option<MetaData> {
        unsafe {
            let metadata = ffi::ChannelCreateMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaData::new(metadata))
            }
        }
    }

    /// Creates source information for the channel.
    pub fn create_source_information(&mut self) -> Option<SourceInformation> {
        unsafe {
            let source_info = ffi::ChannelCreateSourceInformation(self.inner);
            if source_info.is_null() {
                None
            } else {
                Some(SourceInformation::new(source_info))
            }
        }
    }

    /// Creates channel conversion for the channel.
    pub fn create_channel_conversion(&mut self) -> Option<ChannelConversion> {
        unsafe {
            let conversion = ffi::ChannelCreateChannelConversion(self.inner);
            if conversion.is_null() {
                None
            } else {
                Some(ChannelConversion::new(conversion))
            }
        }
    }

    /// Creates channel array for the channel.
    pub fn create_channel_array(&mut self) -> Option<ChannelArray> {
        unsafe {
            let array = ffi::ChannelCreateChannelArray(self.inner);
            if array.is_null() {
                None
            } else {
                Some(ChannelArray::new(array))
            }
        }
    }
}

impl<'a> Deref for Channel<'a> {
    type Target = ChannelRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
