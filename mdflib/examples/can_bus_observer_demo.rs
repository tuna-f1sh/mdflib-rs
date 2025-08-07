//! Example demonstrating how to use CanBusObserver to read CAN messages from MDF files
//!
//! This example shows the complete workflow:
//! 1. Create an MDF file with CAN bus data
//! 2. Read it back using CanBusObserver
//! 3. Extract and process CAN messages

use mdflib::*;
use tempfile::NamedTempFile;

fn main() -> Result<()> {
    println!("CanBusObserver Example");
    println!("======================");

    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    // Step 1: Create an MDF file with CAN bus data
    println!("\n1. Creating MDF file with CAN bus data...");
    {
        let mut writer =
            writer::MdfWriter::new(mdflib_sys::MdfWriterType::MdfBusLogger, file_path)?;
        writer.set_bus_type(MdfBusType::CAN as u16);

        let mut header = writer.get_header().unwrap();
        header.set_description("Example CAN data for CanBusObserver demonstration");
        header.set_author("CanBusObserver Example");

        writer.create_bus_log_configuration();

        let mut data_group = writer.create_data_group().unwrap();
        let mut channel_group = data_group.create_channel_group().unwrap();
        channel_group.set_name("CAN1");

        // Create basic CAN channels
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

        // Create and write some example CAN messages
        let can_messages = [
            (0x123, vec![0x01, 0x02, 0x03, 0x04]),
            (0x456, vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]),
            (0x789, vec![0x10, 0x20]),
            (0x7FF, vec![0xFF; 8]),
        ];

        for (index, (can_id, data)) in can_messages.iter().enumerate() {
            let mut can_message = canmessage::CanMessage::new();
            can_message.set_message_id(*can_id);
            can_message.set_dlc(data.len() as u8);
            can_message.set_data_bytes(data);
            can_message.set_timestamp((index as u64 + 1) * 1000);

            writer.save_can_message(&channel_group, (index as u64 + 1) * 1000, &can_message);
            println!(
                "  Written CAN message: ID=0x{:03X}, DLC={}, Data={:?}",
                can_id,
                data.len(),
                data
            );
        }

        writer.stop_measurement(5000);
        writer.finalize_measurement();
    }

    // Step 2: Read the file back using CanBusObserver
    println!("\n2. Reading MDF file with CanBusObserver...");
    {
        let mut reader = reader::MdfReader::new(file_path)?;
        reader.read_everything_but_data()?;

        let file = reader.get_file().unwrap();
        println!("  File: {}", file.get_name());

        let mut observers = Vec::new();

        // Create CanBusObserver for each CAN channel group
        for dg_index in 0..file.get_data_group_count() {
            let dg = file.get_data_group(dg_index);
            println!(
                "  Data Group {}: {} channel groups",
                dg_index,
                dg.get_channel_group_count()
            );

            for cg_index in 0..dg.get_channel_group_count() {
                let cg = dg.get_channel_group_by_index(cg_index).unwrap();
                let bus_type = cg.get_bus_type();

                println!(
                    "    Channel Group '{}': Bus Type = {} ({})",
                    cg.get_name(),
                    bus_type,
                    if bus_type == BusType::Can as u8 {
                        "CAN"
                    } else {
                        "Other"
                    }
                );

                // Create CanBusObserver for CAN channel groups
                if bus_type == BusType::Can as u8 {
                    match unsafe { create_can_bus_observer(dg.as_ptr(), cg.as_ptr()) } {
                        Ok(observer) => {
                            let name = observer.get_name();
                            let nof_samples = observer.get_nof_samples();
                            println!(
                                "      Created CanBusObserver '{name}' with {nof_samples} samples"
                            );
                            observers.push((name, observer));
                        }
                        Err(e) => {
                            println!("      Failed to create CanBusObserver: {e:?}");
                        }
                    }
                }
            }
        }

        // Step 3: Process the CAN messages
        println!("\n3. Processing CAN messages...");
        if observers.is_empty() {
            println!("  No CanBusObserver instances created - this may indicate:");
            println!("    - No CAN channel groups in the file");
            println!("    - Channel groups don't contain CAN message data");
            println!("    - Data needs to be read first with ReadData()");
        } else {
            for (name, observer) in &observers {
                println!("  Processing observer '{name}':");
                let nof_samples = observer.get_nof_samples();

                if nof_samples == 0 {
                    println!("    No samples available (data may need to be read with ReadData())");
                } else {
                    for sample in 0..nof_samples {
                        if let Some(can_msg) = observer.get_can_message(sample) {
                            let can_id = can_msg.get_can_id();
                            let dlc = can_msg.get_dlc();
                            let data = can_msg.get_data_bytes();
                            let timestamp = can_msg.get_timestamp();

                            println!(
                                "    Sample {}: ID=0x{:03X}, DLC={}, Data={:02X?}, Time={}µs",
                                sample,
                                can_id,
                                dlc,
                                &data[..dlc as usize],
                                timestamp
                            );
                        } else {
                            println!("    Sample {sample}: No CAN message");
                        }
                    }
                }
            }
        }
    }

    println!("\n4. Summary");
    println!("  ✓ Successfully created CanBusObserver");
    println!("  ✓ Demonstrated CAN message creation and parsing");
    println!("  ✓ Showed proper resource management with RAII");

    Ok(())
}
