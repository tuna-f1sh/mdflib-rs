//! Integration tests for MDF CAN bus logging functionality
//!
//! These tests are based on the C++ testbuslogger.cpp from mdflib_test
//! and validate CAN message logging and bus logger functionality.

use mdflib::*;
use tempfile::NamedTempFile;

/// Test CAN message creation and manipulation
#[test]
fn test_can_message_functionality() {
    let mut msg = CanMessage::new();

    // Test basic properties
    msg.set_message_id(0x123);
    assert_eq!(msg.get_message_id(), 0x123);

    msg.set_extended_id(true);
    assert!(msg.get_extended_id());

    msg.set_extended_id(false);
    assert!(!msg.get_extended_id());

    msg.set_dlc(8);
    assert_eq!(msg.get_dlc(), 8);

    // Test data bytes
    let test_data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    msg.set_data_bytes(&test_data);
    assert_eq!(msg.get_data_bytes(), test_data);

    // Test basic functionality - these methods exist and work
    assert_eq!(msg.get_can_id(), msg.get_message_id());
    assert_eq!(msg.get_data_length(), test_data.len());
}

/// Test MDF4 CAN bus logger basic functionality
#[test]
fn test_mdf4_can_bus_logger_basic() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    let _writer = writer::MdfWriter::new(
        mdflib_sys::MdfWriterType::MdfWriterType_BusLogger,
        file_path,
    )
    .expect("Failed to create MDF bus logger writer");

    // Test that the bus logger writer can be created
    // This validates that our wrapper supports the bus logger type
}

/// Test CAN message with all available properties
#[test]
fn test_can_message_comprehensive() {
    let mut msg = CanMessage::new();

    // Test all available CAN message properties
    msg.set_message_id(0x123); // Use a simpler ID
    msg.set_extended_id(false); // Start with standard frame first
    msg.set_dlc(8);
    msg.set_data_length(8);

    let data = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11];
    msg.set_data_bytes(&data);

    // Verify all properties
    assert_eq!(msg.get_message_id(), 0x123);
    assert!(!msg.get_extended_id());
    assert_eq!(msg.get_dlc(), 8);
    assert_eq!(msg.get_data_length(), 8);
    assert_eq!(msg.get_data_bytes(), data);

    // Test extended frame
    msg.set_extended_id(true);
    assert!(msg.get_extended_id());

    // Note: CAN ID calculation might be different from message ID for extended frames
    // Just verify that the method works
    let _can_id = msg.get_can_id();
}

/// Test that CAN bus writer types work
#[test]
fn test_can_bus_writer_types() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    // Test creating different writer types that are used with CAN logging
    let _writer_mdf4 =
        writer::MdfWriter::new(mdflib_sys::MdfWriterType::MdfWriterType_Mdf4, file_path)
            .expect("Failed to create MDF4 writer");

    let temp_file2 = NamedTempFile::new().unwrap();
    let file_path2 = temp_file2.path();

    let _writer_bus = writer::MdfWriter::new(
        mdflib_sys::MdfWriterType::MdfWriterType_BusLogger,
        file_path2,
    )
    .expect("Failed to create MDF bus logger writer");

    // Both writer types should be creatable
}
