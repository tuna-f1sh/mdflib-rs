//! Fallback bindings for when bindgen fails
//! This provides minimal type definitions to get the crate to compile

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_void};

// Opaque pointer types
pub type MdfReader = c_void;
pub type MdfWriter = c_void;
pub type MdfFile = c_void;
pub type IHeader = c_void;
pub type IDataGroup = c_void;
pub type IChannelGroup = c_void;
pub type IChannel = c_void;
pub type CanMessage = c_void;

// Basic enums
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MdfWriterType {
    MDF_WRITER_TYPE_MDF4 = 0,
    MDF_WRITER_TYPE_MDF3 = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ChannelType {
    CHANNEL_TYPE_FIXED_LENGTH = 0,
    CHANNEL_TYPE_VARIABLE_LENGTH = 1,
    CHANNEL_TYPE_MASTER = 2,
    CHANNEL_TYPE_VIRTUAL_MASTER = 3,
    CHANNEL_TYPE_SYNC = 4,
    CHANNEL_TYPE_MAX_LENGTH = 5,
    CHANNEL_TYPE_VIRTUAL_DATA = 6,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ChannelDataType {
    CHANNEL_DATA_TYPE_UNSIGNED_INT = 0,
    CHANNEL_DATA_TYPE_SIGNED_INT = 1,
    CHANNEL_DATA_TYPE_FLOAT = 2,
    CHANNEL_DATA_TYPE_STRING = 3,
    CHANNEL_DATA_TYPE_BYTE_ARRAY = 4,
}

// External function declarations
extern "C" {
    pub fn MdfReaderInit(filename: *const c_char) -> *mut MdfReader;
    pub fn MdfReaderUnInit(reader: *mut MdfReader);
    pub fn MdfReaderIsOk(reader: *mut MdfReader) -> bool;
    pub fn MdfReaderOpen(reader: *mut MdfReader) -> bool;
    pub fn MdfReaderClose(reader: *mut MdfReader);
    pub fn MdfReaderReadHeader(reader: *mut MdfReader) -> bool;
    pub fn MdfReaderReadMeasurementInfo(reader: *mut MdfReader) -> bool;
    pub fn MdfReaderReadEverythingButData(reader: *mut MdfReader) -> bool;
    
    pub fn MdfWriterInit(type_: MdfWriterType, filename: *const c_char) -> *mut MdfWriter;
    pub fn MdfWriterUnInit(writer: *mut MdfWriter);
    pub fn MdfWriterInitMeasurement(writer: *mut MdfWriter) -> bool;
    pub fn MdfWriterFinalizeMeasurement(writer: *mut MdfWriter) -> bool;
    
    pub fn CanMessageInit() -> *mut CanMessage;
    pub fn CanMessageUnInit(can: *mut CanMessage);
}
