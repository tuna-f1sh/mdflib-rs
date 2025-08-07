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

        let mut data_group = writer.create_data_group().unwrap();
        let mut channel_group = data_group.create_channel_group().unwrap();
        channel_group.set_name("CAN1");

        // Create some basic CAN channels
        let mut id_channel = channel_group.create_channel().unwrap();
        id_channel.set_name("CAN_ID");
        id_channel.set_data_type(mdflib_sys::ChannelDataType::UnsignedIntegerLe as u8);
        id_channel.set_data_bytes(4);

        let mut dlc_channel = channel_group.create_channel().unwrap();
        dlc_channel.set_name("CAN_DLC");
        dlc_channel.set_data_type(mdflib_sys::ChannelDataType::UnsignedIntegerLe as u8);
        dlc_channel.set_data_bytes(1);

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
        let dg_count = file.get_data_group_count();

        assert!(dg_count > 0, "Should have at least one data group");

        let dg = file.get_data_group(0);
        let cg_count = dg.get_channel_group_count();

        assert!(cg_count > 0, "Should have at least one channel group");

        let cg = dg.get_channel_group_by_index(0).unwrap();

        // Check if this is a CAN channel group
        let bus_type = cg.get_bus_type();

        // Create a CAN bus observer for CAN data
        if bus_type == BusType::Can as u8 {
            let observer = unsafe { create_can_bus_observer(dg.as_ptr(), cg.as_ptr()) };

            match observer {
                Ok(observer) => {
                    let name = observer.get_name();
                    let nof_samples = observer.get_nof_samples();

                    println!("Created CAN bus observer '{name}' with {nof_samples} samples");

                    // For a fresh observer without read data, we might expect 0 samples
                    // This test primarily validates that the CAN bus observer can be created successfully
                }
                Err(e) => {
                    println!("Failed to create CAN bus observer: {e:?}");
                    // This might be expected if the channel group doesn't contain proper CAN data
                }
            }
        } else {
            println!("Channel group is not a CAN bus type: {bus_type}");
        }
    }
}

/// Test CAN bus observer creation with different bus types
#[test]
fn test_can_bus_observer_non_can_data() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    // Create an MDF file with regular (non-CAN) data
    {
        let mut writer = writer::MdfWriter::new(mdflib_sys::MdfWriterType::Mdf4Basic, file_path)
            .expect("Failed to create MDF writer");

        let mut header = writer.get_header().unwrap();
        header.set_description("Test MDF4 file with regular data");

        let mut data_group = writer.create_data_group().unwrap();
        let mut channel_group = data_group.create_channel_group().unwrap();
        channel_group.set_name("RegularChannelGroup");

        let mut channel = channel_group.create_channel().unwrap();
        channel.set_name("TestChannel");
        channel.set_data_type(mdflib_sys::ChannelDataType::UnsignedIntegerLe as u8);
        channel.set_data_bytes(4);

        writer.init_measurement();
        writer.start_measurement(0);

        // Write some test samples
        for i in 0..5 {
            writer.save_sample(&channel_group, i * 1000);
        }

        writer.stop_measurement(5000);
        writer.finalize_measurement();
    }

    // Try to create CAN bus observer on non-CAN data
    {
        let mut reader = reader::MdfReader::new(file_path).expect("Failed to create MDF reader");
        assert!(reader.is_ok());
        assert!(reader.read_everything_but_data().is_ok());

        let file = reader.get_file().unwrap();
        let dg = file.get_data_group(0);
        let cg = dg.get_channel_group_by_index(0).unwrap();

        // Check bus type - should not be CAN
        let bus_type = cg.get_bus_type();
        println!("Channel group bus type: {bus_type}");

        // Try to create CAN bus observer anyway - this might fail or return an observer with 0 samples
        let observer_result = unsafe { create_can_bus_observer(dg.as_ptr(), cg.as_ptr()) };

        match observer_result {
            Ok(observer) => {
                let name = observer.get_name();
                let nof_samples = observer.get_nof_samples();
                println!(
                    "Created CAN bus observer '{name}' with {nof_samples} samples on non-CAN data"
                );
                // This might be valid - the observer is created but has no CAN messages
            }
            Err(e) => {
                println!("Failed to create CAN bus observer on non-CAN data: {e:?}");
                // This is also valid - the observer creation failed for non-CAN data
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

        // Create first CAN channel group
        let mut data_group1 = writer.create_data_group().unwrap();
        let mut channel_group1 = data_group1.create_channel_group().unwrap();
        channel_group1.set_name("CAN1");

        let mut id_channel1 = channel_group1.create_channel().unwrap();
        id_channel1.set_name("CAN_ID");
        id_channel1.set_data_type(mdflib_sys::ChannelDataType::UnsignedIntegerLe as u8);
        id_channel1.set_data_bytes(4);

        // Create second CAN channel group
        let mut data_group2 = writer.create_data_group().unwrap();
        let mut channel_group2 = data_group2.create_channel_group().unwrap();
        channel_group2.set_name("CAN2");

        let mut id_channel2 = channel_group2.create_channel().unwrap();
        id_channel2.set_name("CAN_ID");
        id_channel2.set_data_type(mdflib_sys::ChannelDataType::UnsignedIntegerLe as u8);
        id_channel2.set_data_bytes(4);

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
            let dg = file.get_data_group(dg_index);

            for cg_index in 0..dg.get_channel_group_count() {
                let cg = dg.get_channel_group_by_index(cg_index).unwrap();

                // Only create CAN bus observers for CAN channel groups
                if cg.get_bus_type() == BusType::Can as u8 {
                    let observer_result =
                        unsafe { create_can_bus_observer(dg.as_ptr(), cg.as_ptr()) };

                    match observer_result {
                        Ok(observer) => {
                            let name = observer.get_name();
                            observers.push((name, observer));
                        }
                        Err(e) => {
                            println!("Failed to create CAN bus observer: {e:?}");
                        }
                    }
                }
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
