use mdflib_sys as ffi;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

use crate::channel::Channel;

/// Represents a channel group in an MDF file.
/// This is a wrapper around the opaque `IChannelGroup` pointer from the C library.
#[derive(Debug)]
pub struct ChannelGroup<'a> {
    pub(crate) inner: *mut ffi::IChannelGroup,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ChannelGroup<'a> {
    pub(crate) fn new(inner: *mut ffi::IChannelGroup) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the index of the channel group.
    pub fn get_index(&self) -> u64 {
        unsafe { ffi::ChannelGroupGetIndex(self.inner) }
    }

    /// Gets the name of the channel group.
    pub fn get_name(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelGroupGetName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelGroupGetName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Sets the name of the channel group.
    pub fn set_name(&mut self, name: &str) {
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            ffi::ChannelGroupSetName(self.inner, c_name.as_ptr());
        }
    }

    /// Gets the description of the channel group.
    pub fn get_description(&self) -> String {
        unsafe {
            let mut len = ffi::ChannelGroupGetDescription(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::ChannelGroupGetDescription(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Sets the description of the channel group.
    pub fn set_description(&mut self, description: &str) {
        let c_description = std::ffi::CString::new(description).unwrap();
        unsafe {
            ffi::ChannelGroupSetDescription(self.inner, c_description.as_ptr());
        }
    }

    /// Gets the number of samples in the channel group.
    pub fn get_nof_samples(&self) -> u64 {
        unsafe { ffi::ChannelGroupGetNofSamples(self.inner) }
    }

    /// Sets the number of samples in the channel group.
    pub fn set_nof_samples(&mut self, samples: u64) {
        unsafe {
            ffi::ChannelGroupSetNofSamples(self.inner, samples);
        }
    }

    /// Gets the number of channels in the channel group.
    pub fn get_channel_count(&self) -> usize {
        unsafe { ffi::ChannelGroupGetChannelCount(self.inner) }
    }

    /// Gets a channel by its index.
    pub fn get_channel(&self, index: usize) -> Option<Channel> {
        unsafe {
            let ch = ffi::ChannelGroupGetChannelByIndex(self.inner, index) as *mut ffi::IChannel;
            if ch.is_null() {
                None
            } else {
                Some(Channel::new(ch))
            }
        }
    }

    /// Creates a new channel in the channel group.
    pub fn create_channel(&mut self) -> Channel {
        unsafe {
            let ch = ffi::ChannelGroupCreateChannel(self.inner);
            Channel::new(ch)
        }
    }
}
