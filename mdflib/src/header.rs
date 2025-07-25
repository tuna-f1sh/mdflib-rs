use mdflib_sys as ffi;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

/// Represents the header of an MDF file.
/// This is a wrapper around the opaque `IHeader` pointer from the C library.
#[derive(Debug)]
pub struct MdfHeader<'a> {
    pub(crate) inner: *const ffi::IHeader,
    _marker: PhantomData<&'a ()>,
}

impl<'a> MdfHeader<'a> {
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

    /// Sets the measurement ID.
    pub fn set_measurement_id(&mut self, id: &str) {
        let c_id = std::ffi::CString::new(id).unwrap();
        unsafe {
            ffi::IHeaderSetMeasurementId(self.inner as *mut ffi::IHeader, c_id.as_ptr());
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

    /// Sets the recorder ID.
    pub fn set_recorder_id(&mut self, id: &str) {
        let c_id = std::ffi::CString::new(id).unwrap();
        unsafe {
            ffi::IHeaderSetRecorderId(self.inner as *mut ffi::IHeader, c_id.as_ptr());
        }
    }

    /// Gets the recorder index.
    pub fn get_recorder_index(&self) -> i64 {
        unsafe { ffi::IHeaderGetRecorderIndex(self.inner) }
    }

    /// Sets the recorder index.
    pub fn set_recorder_index(&mut self, index: i64) {
        unsafe {
            ffi::IHeaderSetRecorderIndex(self.inner as *mut ffi::IHeader, index);
        }
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

    /// Sets the start angle.
    pub fn set_start_angle(&mut self, angle: f64) {
        unsafe {
            ffi::IHeaderSetStartAngle(self.inner as *mut ffi::IHeader, angle);
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

    /// Sets the start distance.
    pub fn set_start_distance(&mut self, distance: f64) {
        unsafe {
            ffi::IHeaderSetStartDistance(self.inner as *mut ffi::IHeader, distance);
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

    /// Sets the author.
    pub fn set_author(&mut self, author: &str) {
        let c_author = std::ffi::CString::new(author).unwrap();
        unsafe {
            ffi::IHeaderSetAuthor(self.inner as *mut ffi::IHeader, c_author.as_ptr());
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

    /// Sets the department.
    pub fn set_department(&mut self, department: &str) {
        let c_department = std::ffi::CString::new(department).unwrap();
        unsafe {
            ffi::IHeaderSetDepartment(self.inner as *mut ffi::IHeader, c_department.as_ptr());
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

    /// Sets the project.
    pub fn set_project(&mut self, project: &str) {
        let c_project = std::ffi::CString::new(project).unwrap();
        unsafe {
            ffi::IHeaderSetProject(self.inner as *mut ffi::IHeader, c_project.as_ptr());
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

    /// Sets the subject.
    pub fn set_subject(&mut self, subject: &str) {
        let c_subject = std::ffi::CString::new(subject).unwrap();
        unsafe {
            ffi::IHeaderSetSubject(self.inner as *mut ffi::IHeader, c_subject.as_ptr());
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

    /// Sets the description.
    pub fn set_description(&mut self, description: &str) {
        let c_description = std::ffi::CString::new(description).unwrap();
        unsafe {
            ffi::IHeaderSetDescription(self.inner as *mut ffi::IHeader, c_description.as_ptr());
        }
    }

    /// Gets the start time.
    pub fn get_start_time(&self) -> u64 {
        unsafe { ffi::IHeaderGetStartTime(self.inner) }
    }

    /// Sets the start time.
    pub fn set_start_time(&mut self, start_time: u64) {
        unsafe {
            ffi::IHeaderSetStartTime(self.inner as *mut ffi::IHeader, start_time);
        }
    }
}
