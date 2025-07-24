use mdflib_sys as ffi;
use std::marker::PhantomData;

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
}
