use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::os::raw::c_char;

use crate::channelgroup::ChannelGroup;

/// Represents an immutable reference to a data group in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct DataGroupRef {
    pub(crate) inner: *const ffi::IDataGroup,
}

impl std::fmt::Display for DataGroupRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DataGroup {{ description: {}, channel_group_count: {} }}",
            self.get_description(),
            self.get_channel_group_count()
        )
    }
}

impl DataGroupRef {
    pub(crate) fn new(inner: *const ffi::IDataGroup) -> Self {
        Self { inner }
    }

    /// Gets the raw pointer to the underlying IDataGroup.
    /// This is used for advanced operations like creating channel observers.
    pub fn as_ptr(&self) -> *const ffi::IDataGroup {
        self.inner
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

    pub fn get_channel_groups(&self) -> Vec<ChannelGroup> {
        let count = self.get_channel_group_count();
        (0..count)
            .filter_map(|i| self.get_channel_group_by_index(i))
            .collect()
    }

    /// Gets a channel group by its index.
    pub fn get_channel_group_by_index(&self, index: usize) -> Option<ChannelGroup> {
        unsafe {
            let cg = ffi::DataGroupGetChannelGroupByIndex(self.inner, index);
            if cg.is_null() {
                None
            } else {
                Some(ChannelGroup::new(cg))
            }
        }
    }

    pub fn get_channel_group(&self, name: &str) -> Option<ChannelGroup> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let cg = ffi::DataGroupGetChannelGroupByName(self.inner, c_name.as_ptr());
            if cg.is_null() {
                None
            } else {
                Some(ChannelGroup::new(cg))
            }
        }
    }
}

/// Represents a mutable reference to a data group in an MDF file.
#[derive(Debug)]
pub struct DataGroup {
    pub(crate) inner: *mut ffi::IDataGroup,
    inner_ref: DataGroupRef,
}

impl DataGroup {
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

    pub fn clear_data(&mut self) {
        unsafe { ffi::DataGroupClearData(self.inner) }
    }
}

impl Deref for DataGroup {
    type Target = DataGroupRef;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
