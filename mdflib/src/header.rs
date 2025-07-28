use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::os::raw::c_char;

use crate::attachment::{Attachment, AttachmentRef};
use crate::datagroup::{DataGroup, DataGroupRef};
use crate::event::{Event, EventRef};
use crate::filehistory::{FileHistory, FileHistoryRef};
use crate::metadata::{MetaData, MetaDataRef};

/// Represents an immutable reference to the header of an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct MdfHeaderRef {
    pub(crate) inner: *const ffi::IHeader,
}

impl MdfHeaderRef {
    pub(crate) fn new(inner: *const ffi::IHeader) -> Self {
        Self { inner }
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

    /// Gets the metadata of the header.
    pub fn get_metadata(&self) -> Option<MetaDataRef> {
        unsafe {
            let metadata = ffi::IHeaderGetMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaDataRef::new(metadata))
            }
        }
    }

    /// Gets the attachments of the header.
    pub fn get_attachments(&self) -> Vec<AttachmentRef> {
        const MAX_ATTACHMENTS: usize = 1000;
        let mut attachments: Vec<*const ffi::IAttachment> = vec![std::ptr::null(); MAX_ATTACHMENTS];
        let count = unsafe {
            ffi::IHeaderGetAttachments(self.inner, attachments.as_mut_ptr(), MAX_ATTACHMENTS)
        };

        attachments.truncate(count);
        attachments
            .into_iter()
            .filter(|&ptr| !ptr.is_null())
            .map(AttachmentRef::new)
            .collect()
    }

    /// Gets the file histories of the header.
    pub fn get_file_histories(&self) -> Vec<FileHistoryRef> {
        const MAX_HISTORIES: usize = 1000;
        let mut histories: Vec<*const ffi::IFileHistory> = vec![std::ptr::null(); MAX_HISTORIES];
        let count = unsafe {
            ffi::IHeaderGetFileHistories(self.inner, histories.as_mut_ptr(), MAX_HISTORIES)
        };

        histories.truncate(count);
        histories
            .into_iter()
            .filter(|&ptr| !ptr.is_null())
            .map(FileHistoryRef::new)
            .collect()
    }

    /// Gets the events of the header.
    pub fn get_events(&self) -> Vec<EventRef> {
        const MAX_EVENTS: usize = 1000;
        let mut events: Vec<*const ffi::IEvent> = vec![std::ptr::null(); MAX_EVENTS];
        let count = unsafe { ffi::IHeaderGetEvents(self.inner, events.as_mut_ptr(), MAX_EVENTS) };

        events.truncate(count);
        events
            .into_iter()
            .filter(|&ptr| !ptr.is_null())
            .map(EventRef::new)
            .collect()
    }
}

/// Represents a mutable reference to the header of an MDF file.
#[derive(Debug)]
pub struct MdfHeader {
    pub(crate) inner: *mut ffi::IHeader,
    inner_ref: MdfHeaderRef,
}

impl MdfHeader {
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

    /// Creates metadata for the header.
    pub fn create_metadata(&mut self) -> Option<MetaData> {
        unsafe {
            let metadata = ffi::IHeaderCreateMetaData(self.inner);
            if metadata.is_null() {
                None
            } else {
                Some(MetaData::new(metadata))
            }
        }
    }

    /// Creates an attachment for the header.
    pub fn create_attachment(&mut self) -> Option<Attachment> {
        unsafe {
            let attachment = ffi::IHeaderCreateAttachment(self.inner);
            if attachment.is_null() {
                None
            } else {
                Some(Attachment::new(attachment))
            }
        }
    }

    /// Creates a file history for the header.
    pub fn create_file_history(&mut self) -> Option<FileHistory> {
        unsafe {
            let file_history = ffi::IHeaderCreateFileHistory(self.inner);
            if file_history.is_null() {
                None
            } else {
                Some(FileHistory::new(file_history))
            }
        }
    }

    /// Creates an event for the header.
    pub fn create_event(&mut self) -> Option<Event> {
        unsafe {
            let event = ffi::IHeaderCreateEvent(self.inner);
            if event.is_null() {
                None
            } else {
                Some(Event::new(event))
            }
        }
    }

    /// Gets all data groups from the header.
    pub fn get_data_groups(&self) -> Vec<DataGroupRef> {
        const MAX_DATA_GROUPS: usize = 1000;
        let mut data_groups: Vec<*const ffi::IDataGroup> = vec![std::ptr::null(); MAX_DATA_GROUPS];
        let count = unsafe {
            ffi::IHeaderGetDataGroups(self.inner, data_groups.as_mut_ptr(), MAX_DATA_GROUPS)
        };

        data_groups.truncate(count);
        data_groups
            .into_iter()
            .filter(|&ptr| !ptr.is_null())
            .map(DataGroupRef::new)
            .collect()
    }

    /// Gets the last data group from the header.
    pub fn get_last_data_group(&self) -> Option<DataGroup> {
        unsafe {
            let data_group = ffi::IHeaderLastDataGroup(self.inner);
            if data_group.is_null() {
                None
            } else {
                Some(DataGroup::new(data_group))
            }
        }
    }
}

impl Deref for MdfHeader {
    type Target = MdfHeaderRef;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
