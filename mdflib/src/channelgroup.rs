use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

use crate::channel::{Channel, ChannelRef};
use crate::metadata::{MetaData, MetaDataRef};
use crate::sourceinformation::{SourceInformation, SourceInformationRef};

/// Represents an immutable reference to a channel group in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct ChannelGroupRef<'a> {
    pub(crate) inner: *const ffi::IChannelGroup,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ChannelGroupRef<'a> {
    pub(crate) fn new(inner: *const ffi::IChannelGroup) -> Self {
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

    /// Gets the number of samples in the channel group.
    pub fn get_nof_samples(&self) -> u64 {
        unsafe { ffi::ChannelGroupGetNofSamples(self.inner) }
    }

    /// Gets the number of channels in the channel group.
    pub fn get_channel_count(&self) -> usize {
        unsafe { ffi::ChannelGroupGetChannelCount(self.inner) }
    }

    /// Gets a channel by its index.
    pub fn get_channel(&self, index: usize) -> Option<ChannelRef> {
        unsafe {
            let ch = ffi::ChannelGroupGetChannelByIndex(self.inner, index);
            if ch.is_null() {
                None
            } else {
                Some(ChannelRef::new(ch))
            }
        }
    }

    /// Gets the metadata of the channel group.
    pub fn get_metadata(&self) -> Option<MetaDataRef> {
        unsafe {
            let metadata = ffi::ChannelGroupGetMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaDataRef::new(metadata))
            }
        }
    }

    /// Gets the source information of the channel group.
    pub fn get_source_information(&self) -> Option<SourceInformationRef> {
        unsafe {
            let source_info = ffi::ChannelGroupGetSourceInformation(self.inner);
            if source_info.is_null() {
                None
            } else {
                Some(SourceInformationRef::new(source_info))
            }
        }
    }
}

/// Represents a mutable reference to a channel group in an MDF file.
#[derive(Debug)]
pub struct ChannelGroup<'a> {
    pub(crate) inner: *mut ffi::IChannelGroup,
    inner_ref: ChannelGroupRef<'a>,
}

impl<'a> ChannelGroup<'a> {
    pub(crate) fn new(inner: *mut ffi::IChannelGroup) -> Self {
        Self {
            inner,
            inner_ref: ChannelGroupRef::new(inner),
        }
    }

    /// Sets the name of the channel group.
    pub fn set_name(&mut self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            ffi::ChannelGroupSetName(self.inner, c_name.as_ptr());
        }
    }

    /// Sets the description of the channel group.
    pub fn set_description(&mut self, description: &str) {
        let c_description = CString::new(description).unwrap();
        unsafe {
            ffi::ChannelGroupSetDescription(self.inner, c_description.as_ptr());
        }
    }

    /// Sets the number of samples in the channel group.
    pub fn set_nof_samples(&mut self, samples: u64) {
        unsafe {
            ffi::ChannelGroupSetNofSamples(self.inner, samples);
        }
    }

    /// Creates a new channel in the channel group.
    pub fn create_channel(&mut self) -> Option<Channel> {
        unsafe {
            let ch = ffi::ChannelGroupCreateChannel(self.inner);
            if ch.is_null() {
                None
            } else {
                Some(Channel::new(ch))
            }
        }
    }

    /// Creates metadata for the channel group.
    pub fn create_metadata(&mut self) -> Option<MetaData> {
        unsafe {
            let metadata = ffi::ChannelGroupCreateMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaData::new(metadata))
            }
        }
    }

    /// Creates source information for the channel group.
    pub fn create_source_information(&mut self) -> Option<SourceInformation> {
        unsafe {
            let source_info = ffi::ChannelGroupCreateSourceInformation(self.inner);
            if source_info.is_null() {
                None
            } else {
                Some(SourceInformation::new(source_info))
            }
        }
    }
}

impl<'a> Deref for ChannelGroup<'a> {
    type Target = ChannelGroupRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
