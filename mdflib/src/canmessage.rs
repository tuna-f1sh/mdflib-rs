//! CAN message wrapper for mdflib CanMessage
//!
//! A CAN message is a message that is sent over a CAN bus. This module provides
//! access to the message's ID, data, and other metadata.
//!
//! ```no_run
//! use mdflib::CanMessage;
//! let mut msg = CanMessage::new();
//!
//! msg.set_message_id(0x123);
//! msg.set_extended_id(false);
//! msg.set_dlc(8);
//! let test_data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
//! msg.set_data_bytes(&test_data);
//! ```

use mdflib_sys as ffi;
use std::marker::PhantomData;
use std::ops::Deref;

/// Represents an immutable reference to a CAN message.
#[derive(Debug, Clone, Copy)]
pub struct CanMessageRef<'a> {
    pub(crate) inner: *const ffi::CanMessage,
    _marker: PhantomData<&'a ()>,
}

impl std::fmt::Display for CanMessageRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CanMessage {{ message_id: {}, can_id: {}, extended_id: {}, dlc: {}, data_length: {}, data_bytes: {:?}, bus_channel: {} }}",
            self.get_message_id(),
            self.get_can_id(),
            self.get_extended_id(),
            self.get_dlc(),
            self.get_data_length(),
            self.get_data_bytes(),
            self.get_bus_channel()
        )
    }
}

impl<'a> CanMessageRef<'a> {
    pub(crate) fn new(inner: *const ffi::CanMessage) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Gets the message ID.
    pub fn get_message_id(&self) -> u32 {
        unsafe { ffi::CanMessageGetMessageId(self.inner) }
    }

    /// Gets the CAN ID.
    pub fn get_can_id(&self) -> u32 {
        unsafe { ffi::CanMessageGetCanId(self.inner) }
    }

    /// Checks if the extended ID is set.
    pub fn get_extended_id(&self) -> bool {
        unsafe { ffi::CanMessageGetExtendedId(self.inner) }
    }

    /// Gets the DLC (Data Length Code).
    pub fn get_dlc(&self) -> u8 {
        unsafe { ffi::CanMessageGetDlc(self.inner) }
    }

    /// Gets the data length.
    pub fn get_data_length(&self) -> usize {
        unsafe { ffi::CanMessageGetDataLength(self.inner) }
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

    /// Gets the bus channel.
    pub fn get_bus_channel(&self) -> u32 {
        unsafe { ffi::CanMessageGetBusChannel(self.inner) }
    }

    /// Gets the timestamp of the message.
    pub fn get_timestamp(&self) -> u64 {
        unsafe { ffi::CanMessageGetTimestamp(self.inner) }
    }

    /// Gets the CRC of the message.
    pub fn get_crc(&self) -> u32 {
        unsafe { ffi::CanMessageGetCrc(self.inner) }
    }
}

/// Represents a mutable CAN message.
#[derive(Debug)]
pub struct CanMessage<'a> {
    pub(crate) inner: *mut ffi::CanMessage,
    inner_ref: CanMessageRef<'a>,
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
                inner_ref: CanMessageRef::new(msg),
            }
        }
    }

    /// Sets the message ID.
    pub fn set_message_id(&mut self, msg_id: u32) {
        unsafe { ffi::CanMessageSetMessageId(self.inner, msg_id) }
    }

    /// Sets the extended ID.
    pub fn set_extended_id(&mut self, extended_id: bool) {
        unsafe { ffi::CanMessageSetExtendedId(self.inner, extended_id) }
    }

    /// Sets the DLC (Data Length Code).
    pub fn set_dlc(&mut self, dlc: u8) {
        unsafe { ffi::CanMessageSetDlc(self.inner, dlc) }
    }

    /// Sets the data length.
    pub fn set_data_length(&mut self, data_length: u32) {
        unsafe { ffi::CanMessageSetDataLength(self.inner, data_length) }
    }

    /// Sets the data bytes.
    pub fn set_data_bytes(&mut self, data: &[u8]) {
        unsafe {
            ffi::CanMessageSetDataBytes(self.inner, data.as_ptr(), data.len());
        }
    }

    /// Sets the bus channel.
    pub fn set_bus_channel(&mut self, bus_channel: u32) {
        unsafe { ffi::CanMessageSetBusChannel(self.inner, bus_channel) }
    }

    /// Sets the timestamp of the message.
    pub fn set_timestamp(&mut self, timestamp: u64) {
        unsafe { ffi::CanMessageSetTimestamp(self.inner, timestamp) }
    }

    /// Sets the CRC of the message.
    pub fn set_crc(&mut self, crc: u32) {
        unsafe { ffi::CanMessageSetCrc(self.inner, crc) }
    }
}

impl<'a> Deref for CanMessage<'a> {
    type Target = CanMessageRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner_ref
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
