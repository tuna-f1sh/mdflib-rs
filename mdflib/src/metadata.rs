//! MetaData wrapper for mdflib IMetaData
//!
//! Metadata is a collection of key-value pairs that can be used to store
//! additional information about an MDF file as XML.

use crate::error::Result;
use crate::etag::ETag;
use mdflib_sys as ffi;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::c_char;

/// Represents an immutable reference to metadata in an MDF file.
#[derive(Debug, Clone, Copy)]
pub struct MetaDataRef<'a> {
    pub(crate) inner: *const ffi::IMetaData,
    _marker: PhantomData<&'a ()>,
}

impl std::fmt::Display for MetaDataRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MetaData {{ xml_snippet: {} }}", self.get_xml_snippet())
    }
}

impl<'a> MetaDataRef<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *const ffi::IMetaData) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets a property as a string.
    pub fn get_property_as_string(&self, index: &str) -> Result<String> {
        let c_index = CString::new(index)?;
        unsafe {
            let mut len = ffi::MetaDataGetPropertyAsString(
                self.inner,
                c_index.as_ptr(),
                std::ptr::null_mut(),
                0,
            );
            if len == 0 {
                return Ok(String::new());
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::MetaDataGetPropertyAsString(self.inner, c_index.as_ptr(), buf.as_mut_ptr(), len);
            Ok(CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned())
        }
    }

    /// Gets a property as a float.
    pub fn get_property_as_float(&self, index: &str) -> Result<f64> {
        let c_index = CString::new(index)?;
        unsafe {
            Ok(ffi::MetaDataGetPropertyAsFloat(
                self.inner,
                c_index.as_ptr(),
            ))
        }
    }

    /// Gets the XML snippet.
    pub fn get_xml_snippet(&self) -> String {
        unsafe {
            let mut len = ffi::MetaDataGetXmlSnippet(self.inner, std::ptr::null_mut(), 0);
            if len == 0 {
                return String::new();
            }
            len += 1; // For null terminator
            let mut buf = vec![0 as c_char; len as usize];
            ffi::MetaDataGetXmlSnippet(self.inner, buf.as_mut_ptr(), len);
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Gets all properties as ETag objects.
    pub fn get_properties(&self) -> Vec<ETag> {
        const MAX_PROPERTIES: usize = 1000;
        let mut properties: Vec<*mut ffi::ETag> = vec![std::ptr::null_mut(); MAX_PROPERTIES];
        let count = unsafe {
            ffi::MetaDataGetProperties(self.inner, properties.as_mut_ptr(), MAX_PROPERTIES)
        };

        properties.truncate(count);
        properties
            .into_iter()
            .filter(|&ptr| !ptr.is_null())
            .map(ETag::from_raw)
            .collect()
    }

    /// Gets common properties as ETag objects.
    pub fn get_common_properties(&self) -> Vec<ETag> {
        const MAX_PROPERTIES: usize = 1000;
        let mut properties: Vec<*mut ffi::ETag> = vec![std::ptr::null_mut(); MAX_PROPERTIES];
        let count = unsafe {
            ffi::MetaDataGetCommonProperties(self.inner, properties.as_mut_ptr(), MAX_PROPERTIES)
        };

        properties.truncate(count);
        properties
            .into_iter()
            .filter(|&ptr| !ptr.is_null())
            .map(ETag::from_raw)
            .collect()
    }
}

/// Represents mutable metadata in an MDF file.
#[derive(Debug)]
pub struct MetaData<'a> {
    pub(crate) inner: *mut ffi::IMetaData,
    inner_ref: MetaDataRef<'a>,
}

impl<'a> MetaData<'a> {
    #[allow(dead_code)]
    pub(crate) fn new(inner: *mut ffi::IMetaData) -> Self {
        Self {
            inner,
            inner_ref: MetaDataRef::new(inner),
        }
    }

    /// Sets a property as a string.
    pub fn set_property_as_string(&mut self, index: &str, prop: &str) -> Result<()> {
        let c_index = CString::new(index)?;
        let c_prop = CString::new(prop)?;
        unsafe {
            ffi::MetaDataSetPropertyAsString(self.inner, c_index.as_ptr(), c_prop.as_ptr());
        }
        Ok(())
    }

    /// Sets a property as a float.
    pub fn set_property_as_float(&mut self, index: &str, prop: f64) -> Result<()> {
        let c_index = CString::new(index)?;
        unsafe {
            ffi::MetaDataSetPropertyAsFloat(self.inner, c_index.as_ptr(), prop);
        }
        Ok(())
    }

    /// Sets the XML snippet.
    pub fn set_xml_snippet(&mut self, xml: &str) -> Result<()> {
        let c_xml = CString::new(xml)?;
        unsafe {
            ffi::MetaDataSetXmlSnippet(self.inner, c_xml.as_ptr());
        }
        Ok(())
    }

    /// Adds a common property.
    pub fn add_common_property(&mut self, tag: &ETag) {
        unsafe {
            ffi::MetaDataAddCommonProperty(self.inner, tag.inner);
        }
    }
}

impl<'a> Deref for MetaData<'a> {
    type Target = MetaDataRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
    }
}
