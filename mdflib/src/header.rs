use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

/// Represents an immutable reference to the header of an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct MdfHeaderRef<'a> {
    pub(crate) inner: *const ffi::IHeader,
    _marker: PhantomData<&'a ()>,
}

impl<'a> MdfHeaderRef<'a> {
    pub(crate) fn new(inner: *const ffi::IHeader) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the measurement ID.
    pub fn get_measurement_id(&self) -> String {
        unsafe {
            let mut len = ffi::IHeaderGetMeasurementId(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::IHeaderGetMeasurementId(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the recorder ID.
    pub fn get_recorder_id(&self) -> String {
        unsafe {
            let mut len = ffi::IHeaderGetRecorderId(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::IHeaderGetRecorderId(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the recorder index.
    pub fn get_recorder_index(&self) -> i64 {
        unsafe { ffi::IHeaderGetRecorderIndex(self.inner) }
    }

    /// Gets the start angle.
    pub fn get_start_angle(&self) -> Option<f64> {
        unsafe {
            let mut angle = 0.0;
            if ffi::IHeaderGetStartAngle(self.inner, &mut angle) {
                Some(angle)
            } else {
                None
            }
        }
    }

    /// Gets the start distance.
    pub fn get_start_distance(&self) -> Option<f64> {
        unsafe {
            let mut distance = 0.0;
            if ffi::IHeaderGetStartDistance(self.inner, &mut distance) {
                Some(distance)
            } else {
                None
            }
        }
    }

    /// Gets the author.
    pub fn get_author(&self) -> String {
        unsafe {
            let mut len = ffi::IHeaderGetAuthor(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::IHeaderGetAuthor(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the department.
    pub fn get_department(&self) -> String {
        unsafe {
            let mut len = ffi::IHeaderGetDepartment(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::IHeaderGetDepartment(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the project.
    pub fn get_project(&self) -> String {
        unsafe {
            let mut len = ffi::IHeaderGetProject(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::IHeaderGetProject(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the subject.
    pub fn get_subject(&self) -> String {
        unsafe {
            let mut len = ffi::IHeaderGetSubject(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::IHeaderGetSubject(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the description.
    pub fn get_description(&self) -> String {
        unsafe {
            let mut len = ffi::IHeaderGetDescription(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::IHeaderGetDescription(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets the start time.
    pub fn get_start_time(&self) -> u64 {
        unsafe { ffi::IHeaderGetStartTime(self.inner) }
    }
}

/// Represents a mutable reference to the header of an MDF file.
#[derive(Debug)]
pub struct MdfHeader<'a> {
    pub(crate) inner: *mut ffi::IHeader,
    inner_ref: MdfHeaderRef<'a>,
}

impl<'a> MdfHeader<'a> {
    pub(crate) fn new(inner: *mut ffi::IHeader) -> Self {
        Self {
            inner,
            inner_ref: MdfHeaderRef::new(inner),
        }
    }

    /// Sets the measurement ID.
    pub fn set_measurement_id(&mut self, id: &str) {
        let c_id = CString::new(id).unwrap();
        unsafe {
            ffi::IHeaderSetMeasurementId(self.inner, c_id.as_ptr());
        }
    }

    /// Sets the recorder ID.
    pub fn set_recorder_id(&mut self, id: &str) {
        let c_id = CString::new(id).unwrap();
        unsafe {
            ffi::IHeaderSetRecorderId(self.inner, c_id.as_ptr());
        }
    }

    /// Sets the recorder index.
    pub fn set_recorder_index(&mut self, index: i64) {
        unsafe {
            ffi::IHeaderSetRecorderIndex(self.inner, index);
        }
    }

    /// Sets the start angle.
    pub fn set_start_angle(&mut self, angle: f64) {
        unsafe {
            ffi::IHeaderSetStartAngle(self.inner, angle);
        }
    }

    /// Sets the start distance.
    pub fn set_start_distance(&mut self, distance: f64) {
        unsafe {
            ffi::IHeaderSetStartDistance(self.inner, distance);
        }
    }

    /// Sets the author.
    pub fn set_author(&mut self, author: &str) {
        let c_author = CString::new(author).unwrap();
        unsafe {
            ffi::IHeaderSetAuthor(self.inner, c_author.as_ptr());
        }
    }

    /// Sets the department.
    pub fn set_department(&mut self, department: &str) {
        let c_department = CString::new(department).unwrap();
        unsafe {
            ffi::IHeaderSetDepartment(self.inner, c_department.as_ptr());
        }
    }

    /// Sets the project.
    pub fn set_project(&mut self, project: &str) {
        let c_project = CString::new(project).unwrap();
        unsafe {
            ffi::IHeaderSetProject(self.inner, c_project.as_ptr());
        }
    }

    /// Sets the subject.
    pub fn set_subject(&mut self, subject: &str) {
        let c_subject = CString::new(subject).unwrap();
        unsafe {
            ffi::IHeaderSetSubject(self.inner, c_subject.as_ptr());
        }
    }

    /// Sets the description.
    pub fn set_description(&mut self, description: &str) {
        let c_description = CString::new(description).unwrap();
        unsafe {
            ffi::IHeaderSetDescription(self.inner, c_description.as_ptr());
        }
    }

    /// Sets the start time.
    pub fn set_start_time(&mut self, start_time: u64) {
        unsafe {
            ffi::IHeaderSetStartTime(self.inner, start_time);
        }
    }
}

impl<'a> Deref for MdfHeader<'a> {
    type Target = MdfHeaderRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}