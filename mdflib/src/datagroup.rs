use mdflib_sys as ffi;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

use crate::channelgroup::ChannelGroup;

/// Represents a data group in an MDF file.
/// This is a wrapper around the opaque `IDataGroup` pointer from the C library.
#[derive(Debug)]
pub struct DataGroup<'a> {
    pub(crate) inner: *mut ffi::IDataGroup,
    _marker: PhantomData<&'a ()>,
}

impl<'a> DataGroup<'a> {
    pub(crate) fn new(inner: *mut ffi::IDataGroup) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn get_description(&self) -> String {
        unsafe {
            let mut len = ffi::DataGroupGetDescription(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::DataGroupGetDescription(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    pub fn set_description(&mut self, description: &str) {
        let c_description = std::ffi::CString::new(description).unwrap();
        unsafe { ffi::DataGroupSetDescription(self.inner, c_description.as_ptr()) }
    }

    pub fn get_channel_group_count(&self) -> usize {
        unsafe { ffi::DataGroupGetChannelGroupCount(self.inner) }
    }

    /// Gets a channel group by its index.
    pub fn get_channel_group(&self, index: usize) -> Option<ChannelGroup> {
        unsafe {
            let cg =
                ffi::DataGroupGetChannelGroupByIndex(self.inner, index) as *mut ffi::IChannelGroup;
            if cg.is_null() {
                None
            } else {
                Some(ChannelGroup::new(cg))
            }
        }
    }
}
