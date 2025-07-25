use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

/// Represents an immutable reference to a channel in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct ChannelRef<'a> {
    pub(crate) inner: *const ffi::IChannel,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ChannelRef<'a> {
    pub(crate) fn new(inner: *const ffi::IChannel) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
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
}

impl<'a> Deref for Channel<'a> {
    type Target = ChannelRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
