use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

use crate::channelgroup::{ChannelGroup, ChannelGroupRef};

/// Represents an immutable reference to a data group in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct DataGroupRef<'a> {
    pub(crate) inner: *const ffi::IDataGroup,
    _marker: PhantomData<&'a ()>,
}

impl<'a> DataGroupRef<'a> {
    pub(crate) fn new(inner: *const ffi::IDataGroup) -> Self {
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

    pub fn get_channel_group_count(&self) -> usize {
        unsafe { ffi::DataGroupGetChannelGroupCount(self.inner) }
    }

    /// Gets a channel group by its index.
    pub fn get_channel_group(&self, index: usize) -> Option<ChannelGroupRef> {
        unsafe {
            let cg = ffi::DataGroupGetChannelGroupByIndex(self.inner, index);
            if cg.is_null() {
                None
            } else {
                Some(ChannelGroupRef::new(cg))
            }
        }
    }
}

/// Represents a mutable reference to a data group in an MDF file.
#[derive(Debug)]
pub struct DataGroup<'a> {
    pub(crate) inner: *mut ffi::IDataGroup,
    inner_ref: DataGroupRef<'a>,
}

impl<'a> DataGroup<'a> {
    pub(crate) fn new(inner: *mut ffi::IDataGroup) -> Self {
        Self {
            inner,
            inner_ref: DataGroupRef::new(inner),
        }
    }

    pub fn set_description(&mut self, description: &str) {
        let c_description = CString::new(description).unwrap();
        unsafe { ffi::DataGroupSetDescription(self.inner, c_description.as_ptr()) }
    }

    pub fn create_channel_group(&mut self) -> Option<ChannelGroup> {
        unsafe {
            let cg = ffi::DataGroupCreateChannelGroup(self.inner);
            if cg.is_null() {
                None
            } else {
                Some(ChannelGroup::new(cg))
            }
        }
    }
}

impl<'a> Deref for DataGroup<'a> {
    type Target = DataGroupRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
