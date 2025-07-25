use mdflib_sys as ffi;
use std::ffi::CStr;
use std::ops::Deref;
use std::os::raw::c_char;

use crate::{DataGroup, DataGroupRef, MdfHeaderRef};

#[derive(Debug, Clone, Copy)]
pub struct MdfFileRef {
    pub(crate) inner: *const ffi::MdfFile,
}

impl MdfFileRef {
    pub fn new(ptr: *const ffi::MdfFile) -> Self {
        Self { inner: ptr }
    }

    pub fn get_name(&self) -> String {
        let mut name_buffer = [0 as c_char; 1024];
        unsafe {
            ffi::MdfFileGetName(self.inner, name_buffer.as_mut_ptr(), name_buffer.len());
            CStr::from_ptr(name_buffer.as_ptr())
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn get_file_name(&self) -> String {
        let mut name_buffer = [0 as c_char; 1024];
        unsafe {
            ffi::MdfFileGetFileName(self.inner, name_buffer.as_mut_ptr(), name_buffer.len());
            CStr::from_ptr(name_buffer.as_ptr())
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn get_version(&self) -> String {
        let mut version_buffer = [0 as c_char; 1024];
        unsafe {
            ffi::MdfFileGetVersion(
                self.inner,
                version_buffer.as_mut_ptr(),
                version_buffer.len(),
            );
            CStr::from_ptr(version_buffer.as_ptr())
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn get_main_version(&self) -> i32 {
        unsafe { ffi::MdfFileGetMainVersion(self.inner) }
    }

    pub fn get_minor_version(&self) -> i32 {
        unsafe { ffi::MdfFileGetMinorVersion(self.inner) }
    }

    pub fn get_header(&self) -> MdfHeaderRef {
        unsafe { MdfHeaderRef::new(ffi::MdfFileGetHeader(self.inner)) }
    }

    pub fn is_mdf4(&self) -> bool {
        unsafe { ffi::MdfFileGetIsMdf4(self.inner) }
    }

    pub fn get_data_group_count(&self) -> usize {
        unsafe { ffi::MdfFileGetDataGroupCount(self.inner) }
    }

    pub fn get_data_group(&self, index: usize) -> DataGroupRef {
        unsafe { DataGroupRef::new(ffi::MdfFileGetDataGroupByIndex(self.inner, index)) }
    }
}

#[derive(Debug)]
pub struct MdfFile {
    pub(crate) inner: *mut ffi::MdfFile,
    inner_ref: MdfFileRef,
}

impl MdfFile {
    pub fn new(ptr: *mut ffi::MdfFile) -> Self {
        Self {
            inner: ptr,
            inner_ref: MdfFileRef::new(ptr),
        }
    }

    pub fn create_data_group(&mut self) -> DataGroup {
        unsafe { DataGroup::new(ffi::MdfFileCreateDataGroup(self.inner)) }
    }
}

impl Deref for MdfFile {
    type Target = MdfFileRef;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
