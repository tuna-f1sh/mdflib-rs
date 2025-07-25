use mdflib_sys as ffi;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct CanMessage<'a> {
    pub(crate) inner: *mut ffi::CanMessage,
    _marker: PhantomData<&'a ()>,
}

impl Default for CanMessage<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CanMessage<'a> {
    /// Creates a new `CanMessage`.
    pub fn new() -> Self {
        unsafe {
            let msg = ffi::CanMessageInit();
            Self {
                inner: msg,
                _marker: PhantomData,
            }
        }
    }

    /// Gets the message ID.
    pub fn get_message_id(&self) -> u32 {
        unsafe { ffi::CanMessageGetMessageId(self.inner) }
    }

    /// Sets the message ID.
    pub fn set_message_id(&mut self, msg_id: u32) {
        unsafe { ffi::CanMessageSetMessageId(self.inner, msg_id) }
    }

    /// Gets the CAN ID.
    pub fn get_can_id(&self) -> u32 {
        unsafe { ffi::CanMessageGetCanId(self.inner) }
    }

    /// Checks if the extended ID is set.
    pub fn get_extended_id(&self) -> bool {
        unsafe { ffi::CanMessageGetExtendedId(self.inner) }
    }

    /// Sets the extended ID.
    pub fn set_extended_id(&mut self, extended_id: bool) {
        unsafe { ffi::CanMessageSetExtendedId(self.inner, extended_id) }
    }

    /// Gets the DLC (Data Length Code).
    pub fn get_dlc(&self) -> u8 {
        unsafe { ffi::CanMessageGetDlc(self.inner) }
    }

    /// Sets the DLC (Data Length Code).
    pub fn set_dlc(&mut self, dlc: u8) {
        unsafe { ffi::CanMessageSetDlc(self.inner, dlc) }
    }

    /// Gets the data length.
    pub fn get_data_length(&self) -> usize {
        unsafe { ffi::CanMessageGetDataLength(self.inner) }
    }

    /// Sets the data length.
    pub fn set_data_length(&mut self, data_length: u32) {
        unsafe { ffi::CanMessageSetDataLength(self.inner, data_length) }
    }

    /// Gets the data bytes.
    pub fn get_data_bytes(&self) -> Vec<u8> {
        unsafe {
            let len = ffi::CanMessageGetDataLength(self.inner);
            let mut buf = vec![0u8; len];
            ffi::CanMessageGetDataBytes(self.inner, buf.as_mut_ptr(), len);
            buf
        }
    }

    /// Sets the data bytes.
    pub fn set_data_bytes(&mut self, data: &[u8]) {
        unsafe {
            ffi::CanMessageSetDataBytes(self.inner, data.as_ptr(), data.len());
        }
    }
}

impl<'a> Drop for CanMessage<'a> {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                ffi::CanMessageUnInit(self.inner);
            }
        }
    }
}
