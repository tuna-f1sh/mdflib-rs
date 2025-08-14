//! Integration tests for CanBusObserver functionality
//!
//! These tests demonstrate the usage of CanBusObserver for reading CAN message data
//! from MDF files, following the pattern shown in the mdflib documentation.

use mdflib::*;
use tempfile::NamedTempFile;

/// Test basic CAN bus observer functionality
#[test]
fn test_can_bus_observer_basic() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    // First, create an MDF file with CAN bus data
    {
        let mut writer = writer::MdfWriter::new(mdflib_sys::MdfWriterType::MdfBusLogger, file_path)
            .expect("Failed to create MDF writer");

        writer.set_bus_type(MdfBusType::CAN as u16);

        let mut header = writer.get_header().unwrap();
        header.set_description("Test MDF4 file for CAN bus observer");

        // Create bus log configuration
        writer.create_bus_log_configuration();

        let header = writer.get_header().unwrap();
        let last_dg = header.get_last_data_group().unwrap();
        let channel_group = last_dg.get_channel_group("_DataFrame").unwrap();

        writer.init_measurement();
        writer.start_measurement(0);

        // Create and write some CAN messages
        let mut can_message = canmessage::CanMessage::new();
        can_message.set_message_id(0x123);
        can_message.set_dlc(8);
        can_message.set_data_bytes(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);

        for i in 0..10 {
            can_message.set_timestamp(i * 1000);
            writer.save_can_message(&channel_group, i * 1000, &can_message);
        }

        writer.stop_measurement(10000);
        writer.finalize_measurement();
    }

    // Now read the file back using CAN bus observers
    {
        let mut reader = reader::MdfReader::new(file_path).expect("Failed to create MDF reader");
        assert!(reader.is_ok());
        assert!(reader.read_everything_but_data().is_ok());

        let file = reader.get_file().unwrap();

        for mut dg in file.get_data_groups() {
            for cg in dg.get_channel_groups() {
                // Only create CAN bus observers for CAN channel groups
                if cg.get_bus_type() == BusType::Can as u8 {
                    let observer =
                        unsafe { create_can_bus_observer(dg.as_ptr(), cg.as_ptr()).unwrap() };
                    reader.read_data(&mut dg).unwrap();
                    let name = observer.get_name();
                    let nof_samples = observer.get_nof_samples();

                    if name.contains("DataFrame") {
                        assert!(
                            nof_samples == 10,
                            "Observer '{}' samples {nof_samples} != 10",
                            name
                        );
                    }

                    // Process the samples
                    for sample in 0..nof_samples {
                        if let Some(can_msg) = observer.get_can_message(sample) {
                            println!(
                                "Sample {}: CAN ID=0x{:X}, DLC={}, Data={:?}",
                                sample,
                                can_msg.get_can_id(),
                                can_msg.get_dlc(),
                                can_msg.get_data_bytes()
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Test multiple CAN bus observers - similar to the documentation example
#[test]
fn test_can_bus_observer_multiple() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    // Create an MDF file with multiple channel groups
    {
        let mut writer = writer::MdfWriter::new(mdflib_sys::MdfWriterType::MdfBusLogger, file_path)
            .expect("Failed to create MDF writer");

        writer.set_bus_type(MdfBusType::CAN as u16);

        let mut header = writer.get_header().unwrap();
        header.set_description("Test MDF4 file with multiple CAN channels");

        writer.create_bus_log_configuration();
        writer.create_data_group();
        writer.create_bus_log_configuration();

        let file = writer.get_file().unwrap();
        let can1_dg = file.get_data_group(0).unwrap();
        let can2_dg = file.get_data_group(1).unwrap();
        let channel_group1 = can1_dg.get_channel_group("_DataFrame").unwrap();
        let channel_group2 = can2_dg.get_channel_group("_DataFrame").unwrap();

        writer.init_measurement();
        writer.start_measurement(0);

        // Create and write CAN messages to both groups
        let mut can_message = canmessage::CanMessage::new();
        can_message.set_message_id(0x123);
        can_message.set_dlc(4);
        can_message.set_data_bytes(&[0x01, 0x02, 0x03, 0x04]);

        for i in 0..5 {
            can_message.set_timestamp(i * 1000);
            writer.save_can_message(&channel_group1, i * 1000, &can_message);
            writer.save_can_message(&channel_group2, i * 1000, &can_message);
        }

        writer.stop_measurement(5000);
        writer.finalize_measurement();
    }

    // Read the file and create observers for all CAN channel groups
    {
        let mut reader = reader::MdfReader::new(file_path).expect("Failed to create MDF reader");
        assert!(reader.is_ok());
        assert!(reader.read_everything_but_data().is_ok());

        let file = reader.get_file().unwrap();

        // Following the pattern from the documentation example:
        // Create observers for all CAN channel groups
        let mut observers = Vec::new();

        for dg_index in 0..file.get_data_group_count() {
            let mut dg = file.get_data_group(dg_index).unwrap();

            for cg_index in 0..dg.get_channel_group_count() {
                let cg = dg.get_channel_group_by_index(cg_index).unwrap();

                // Only create CAN bus observers for CAN channel groups
                if cg.get_bus_type() == BusType::Can as u8 {
                    let observer =
                        unsafe { create_can_bus_observer(dg.as_ptr(), cg.as_ptr()).unwrap() };
                    let name = observer.get_name();
                    observers.push((name, observer));
                }

                reader.read_data(&mut dg).unwrap();
            }
        }

        // Process all the observers
        assert!(
            !observers.is_empty(),
            "Should have created some CAN bus observers"
        );

        for (name, observer) in &observers {
            let nof_samples = observer.get_nof_samples();

            println!("Created CAN bus observer '{name}' with {nof_samples} samples");

            if name.contains("DataFrame") {
                assert!(
                    nof_samples == 5,
                    "Observer '{}' samples {nof_samples} != 5",
                    name
                );
            }

            // Test that the basic observer methods work
            for sample in 0..nof_samples {
                if let Some(can_msg) = observer.get_can_message(sample) {
                    println!(
                        "Sample {}: CAN ID=0x{:X}, DLC={}",
                        sample,
                        can_msg.get_can_id(),
                        can_msg.get_dlc()
                    );
                }
            }
        }
    }
}
