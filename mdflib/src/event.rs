//! Event wrapper for mdflib
//!
//! This module provides safe Rust wrappers around the mdflib IEvent functionality.

use crate::error::Result;
use crate::metadata::MetaDataRef;
use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

/// Represents an immutable reference to an event in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct EventRef<'a> {
    pub(crate) inner: *const ffi::IEvent,
    _marker: PhantomData<&'a ()>,
}

impl<'a> EventRef<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *const ffi::IEvent) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the index of the event.
    pub fn get_index(&self) -> u64 {
        unsafe { ffi::EventGetIndex(self.inner) }
    }

    /// Gets the name of the event.
    pub fn get_name(&self) -> String {
        unsafe {
            let mut len = ffi::EventGetName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::EventGetName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the description of the event.
    pub fn get_description(&self) -> String {
        unsafe {
            let mut len = ffi::EventGetDescription(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::EventGetDescription(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the group name of the event.
    pub fn get_group_name(&self) -> String {
        unsafe {
            let mut len = ffi::EventGetGroupName(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::EventGetGroupName(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the type of the event.
    pub fn get_type(&self) -> u8 {
        unsafe { ffi::EventGetType(self.inner) }
    }

    /// Gets the sync type of the event.
    pub fn get_sync(&self) -> u8 {
        unsafe { ffi::EventGetSync(self.inner) }
    }

    /// Gets the range type of the event.
    pub fn get_range(&self) -> u8 {
        unsafe { ffi::EventGetRange(self.inner) }
    }

    /// Gets the cause of the event.
    pub fn get_cause(&self) -> u8 {
        unsafe { ffi::EventGetCause(self.inner) }
    }

    /// Gets the creator index of the event.
    pub fn get_creator_index(&self) -> u16 {
        unsafe { ffi::EventGetCreatorIndex(self.inner) }
    }

    /// Gets the sync value of the event.
    pub fn get_sync_value(&self) -> i64 {
        unsafe { ffi::EventGetSyncValue(self.inner) }
    }

    /// Gets the sync factor of the event.
    pub fn get_sync_factor(&self) -> f64 {
        unsafe { ffi::EventGetSyncFactor(self.inner) }
    }

    /// Gets the pre-trigger time of the event.
    pub fn get_pre_trig(&self) -> f64 {
        unsafe { ffi::EventGetPreTrig(self.inner) }
    }

    /// Gets the post-trigger time of the event.
    pub fn get_post_trig(&self) -> f64 {
        unsafe { ffi::EventGetPostTrig(self.inner) }
    }

    /// Gets the metadata of the event.
    pub fn get_metadata(&self) -> Option<MetaDataRef<'a>> {
        unsafe {
            let metadata = ffi::EventGetMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaDataRef::new(metadata))
            }
        }
    }
}

/// Represents a mutable event in an MDF file.
#[derive(Debug)]
pub struct Event<'a> {
    pub(crate) inner: *mut ffi::IEvent,
    inner_ref: EventRef<'a>,
}

impl<'a> Event<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *mut ffi::IEvent) -> Self {
        Self {
            inner,
            inner_ref: EventRef::new(inner),
        }
    }

    /// Sets the name of the event.
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name)?;
        unsafe {
            ffi::EventSetName(self.inner, c_name.as_ptr());
        }
        Ok(())
    }

    /// Sets the description of the event.
    pub fn set_description(&mut self, description: &str) -> Result<()> {
        let c_description = CString::new(description)?;
        unsafe {
            ffi::EventSetDescription(self.inner, c_description.as_ptr());
        }
        Ok(())
    }

    /// Sets the group name of the event.
    pub fn set_group_name(&mut self, group: &str) -> Result<()> {
        let c_group = CString::new(group)?;
        unsafe {
            ffi::EventSetGroupName(self.inner, c_group.as_ptr());
        }
        Ok(())
    }

    /// Sets the type of the event.
    pub fn set_type(&mut self, event_type: u8) {
        unsafe {
            ffi::EventSetType(self.inner, event_type);
        }
    }

    /// Sets the sync type of the event.
    pub fn set_sync(&mut self, sync_type: u8) {
        unsafe {
            ffi::EventSetSync(self.inner, sync_type);
        }
    }

    /// Sets the range type of the event.
    pub fn set_range(&mut self, range_type: u8) {
        unsafe {
            ffi::EventSetRange(self.inner, range_type);
        }
    }

    /// Sets the cause of the event.
    pub fn set_cause(&mut self, cause: u8) {
        unsafe {
            ffi::EventSetCause(self.inner, cause);
        }
    }

    /// Sets the creator index of the event.
    pub fn set_creator_index(&mut self, index: u16) {
        unsafe {
            ffi::EventSetCreatorIndex(self.inner, index);
        }
    }

    /// Sets the sync value of the event.
    pub fn set_sync_value(&mut self, value: i64) {
        unsafe {
            ffi::EventSetSyncValue(self.inner, value);
        }
    }

    /// Sets the sync factor of the event.
    pub fn set_sync_factor(&mut self, factor: f64) {
        unsafe {
            ffi::EventSetSyncFactor(self.inner, factor);
        }
    }

    /// Sets the pre-trigger time of the event.
    pub fn set_pre_trig(&mut self, time: f64) {
        unsafe {
            ffi::EventSetPreTrig(self.inner, time);
        }
    }

    /// Sets the post-trigger time of the event.
    pub fn set_post_trig(&mut self, time: f64) {
        unsafe {
            ffi::EventSetPostTrig(self.inner, time);
        }
    }
}

impl<'a> Deref for Event<'a> {
    type Target = EventRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
