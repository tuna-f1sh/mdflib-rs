//! Integration tests for IChannelObserver functionality
//!
//! These tests demonstrate the usage of IChannelObserver for reading MDF file data,
//! following the pattern shown in the mdflib documentation.

use mdflib::*;
use tempfile::NamedTempFile;

/// Test basic channel observer functionality
#[test]
fn test_channel_observer_basic() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    // First, create an MDF file with some test data
    {
        let mut writer = writer::MdfWriter::new(mdflib_sys::MdfWriterType::Mdf4Basic, file_path)
            .expect("Failed to create MDF writer");

        let mut header = writer.get_header().unwrap();
        header.set_description("Test MDF4 file for channel observer");

        let mut data_group = writer.create_data_group().unwrap();
        let mut channel_group = data_group.create_channel_group().unwrap();
        channel_group.set_name("TestChannelGroup");

        let mut channel = channel_group.create_channel().unwrap();
        channel.set_name("TestChannel");
        channel.set_data_type(mdflib_sys::ChannelDataType::UnsignedIntegerLe as u8);
        channel.set_data_bytes(4);

        writer.init_measurement();
        writer.start_measurement(0);

        // Write some test samples
        for i in 0..10 {
            writer.save_sample(&channel_group, i * 1000);
        }

        writer.stop_measurement(10000);
        writer.finalize_measurement();
    }

    // Now read the file back using channel observers
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
        let channel_count = cg.get_channel_count();

        assert!(channel_count > 0, "Should have at least one channel");

        let channel = cg.get_channel(0).unwrap();

        // Read the actual data into the data group
        // Note: For a proper test, we would need a DataGroup (not DataGroupRef)
        // and actual sample data. For now, we just test channel observer creation

        // Create a channel observer to read the sample data
        let observer =
            unsafe { create_channel_observer(dg.as_ptr(), cg.as_ptr(), channel.as_ptr()) }
                .expect("Should be able to create channel observer");

        let nof_samples = observer.get_nof_samples();

        // For a fresh observer without read data, we expect 0 samples
        // This test primarily validates that the channel observer can be created successfully
        println!("Created channel observer with {nof_samples} samples");
    }
}

/// Test channel observer with multiple channels - similar to the documentation example
#[test]
fn test_channel_observer_multiple_channels() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    // Create an MDF file with multiple channels
    {
        let mut writer = writer::MdfWriter::new(mdflib_sys::MdfWriterType::Mdf4Basic, file_path)
            .expect("Failed to create MDF writer");

        let mut header = writer.get_header().unwrap();
        header.set_description("Test MDF4 file with multiple channels");

        let mut data_group = writer.create_data_group().unwrap();
        let mut channel_group = data_group.create_channel_group().unwrap();
        channel_group.set_name("MultiChannelGroup");

        // Create temperature channel
        let mut temp_channel = channel_group.create_channel().unwrap();
        temp_channel.set_name("Temperature");
        temp_channel.set_unit("°C");
        temp_channel.set_data_type(mdflib_sys::ChannelDataType::FloatLe as u8);
        temp_channel.set_data_bytes(8);

        // Create pressure channel
        let mut pressure_channel = channel_group.create_channel().unwrap();
        pressure_channel.set_name("Pressure");
        pressure_channel.set_unit("bar");
        pressure_channel.set_data_type(mdflib_sys::ChannelDataType::FloatLe as u8);
        pressure_channel.set_data_bytes(8);

        writer.init_measurement();
        writer.start_measurement(0);

        // Write some test samples (simulate temperature and pressure data)
        for i in 0..20 {
            writer.save_sample(&channel_group, i * 1000);
        }

        writer.stop_measurement(20000);
        writer.finalize_measurement();
    }

    // Read the file and create observers for all channels
    {
        let mut reader = reader::MdfReader::new(file_path).expect("Failed to create MDF reader");
        assert!(reader.is_ok());
        assert!(reader.read_everything_but_data().is_ok());

        let file = reader.get_file().unwrap();

        // This follows the pattern from the documentation example:
        // for (auto* dg4 : dg_list) {
        //   ChannelObserverList subscriber_list;
        //   const auto cg_list = dg4->ChannelGroups();
        //   for (const auto* cg4 : cg_list ) {
        //     const auto cn_list = cg4->Channels();
        //     for (const auto* cn4 : cn_list) {
        //       auto sub = CreateChannelObserver(*dg4, *cg4, *cn4);
        //       subscriber_list.emplace_back(std::move(sub));
        //     }
        //   }

        let mut observers = Vec::new();

        for dg_index in 0..file.get_data_group_count() {
            let dg = file.get_data_group(dg_index);

            // Note: We can't read data from DataGroupRef, only from DataGroup
            // This test demonstrates creating channel observers for the structure

            for cg_index in 0..dg.get_channel_group_count() {
                let cg = dg.get_channel_group_by_index(cg_index).unwrap();

                for ch_index in 0..cg.get_channel_count() {
                    let channel = cg.get_channel(ch_index).unwrap();

                    // Create a channel observer for each channel
                    let observer = unsafe {
                        create_channel_observer(dg.as_ptr(), cg.as_ptr(), channel.as_ptr())
                    }
                    .expect("Should be able to create channel observer");

                    observers.push((channel.get_name(), observer));
                }
            }
        }

        // Process all the observers - similar to the documentation example:
        // for (auto& obs : subscriber_list) {
        //   for (size_t sample = 0; sample < obs->NofSamples(); ++sample) {
        //     const auto channel_valid = obs->GetChannelValue(sample, channel_value);
        //     const auto eng_valid = obs->GetEngValue(sample, eng_value);
        //   }
        // }

        assert!(!observers.is_empty(), "Should have created some observers");

        for (channel_name, observer) in &observers {
            let nof_samples = observer.get_nof_samples();

            println!("Created observer for channel '{channel_name}' with {nof_samples} samples");

            // Test that the basic observer methods work (even with 0 samples)
            let all_channel_values = observer.get_all_channel_values();
            let all_eng_values = observer.get_all_eng_values();

            assert_eq!(all_channel_values.len(), nof_samples);
            assert_eq!(all_eng_values.len(), nof_samples);
        }
    }
}
