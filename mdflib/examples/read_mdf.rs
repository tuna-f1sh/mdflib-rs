//! Example: Reading an MDF file
use mdflib::{create_channel_observer, ChannelObserver, MdfReader, Result};
use std::env;

pub fn set_env_logger() {
    // Initialize the logger with the default settings
    env_logger::init();

    // Set the log callback to use the env_logger
    mdflib::log::set_log_callback_1(Some(mdflib::log::log_callback));
}

fn main() -> Result<()> {
    set_env_logger();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <mdf_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    println!("Opening MDF file: {filename}");

    let mut reader = MdfReader::new(filename)?;

    if !reader.is_ok() {
        eprintln!("Reader is not in a valid state");
        return Ok(());
    }

    if !reader.is_finalized() {
        eprintln!("Reader is not finalized");
    }

    println!("Reading file metadata...");
    reader.read_everything_but_data()?;

    if let Some(file) = reader.get_file() {
        println!("File: {file}");

        println!("\nAttachments:");
        for attachment in file.get_attachments() {
            println!("  {attachment}");
        }

        if let Some(header) = reader.get_header() {
            println!("\nHeader: {header}");

            println!("\nFile Histories:");
            for history in header.get_file_histories() {
                println!("  {history}");
            }

            println!("\nEvents:");
            for event in header.get_events() {
                println!("  {event}");
            }
        }

        println!("\nData Groups ({})", file.get_data_group_count());
        let mut data_groups = file.get_data_groups();
        for (i, data_group) in data_groups.iter_mut().enumerate() {
            println!("\n  Data Group {}: {:?}", i, &*data_group);

            let mut observers: Vec<(String, ChannelObserver)> = Vec::new();

            println!(
                "    Channel Groups ({})",
                data_group.get_channel_group_count()
            );
            let channel_groups = data_group.get_channel_groups();
            for (j, channel_group) in channel_groups.iter().enumerate() {
                println!("      Channel Group {j}: {channel_group:?}");

                if let Some(si) = channel_group.get_source_information() {
                    println!("        Source Info: {si}");
                }

                println!("        Channels ({})", channel_group.get_channel_count());
                let channels = channel_group.get_channels();
                for (k, channel) in channels.iter().enumerate() {
                    println!("          Channel {k}: {channel}");
                    if let Some(si) = channel.get_source_information() {
                        println!("            Source Info: {si}");
                    }
                    if let Some(cc) = channel.get_channel_conversion() {
                        println!("            Conversion: {cc}");
                    }
                    if let Some(ca) = channel.get_channel_array() {
                        println!("            Array: {ca}");
                    }

                    // Create and store channel observer
                    let observer = unsafe {
                        create_channel_observer(
                            data_group.as_ptr(),
                            channel_group.as_ptr(),
                            channel.as_ptr(),
                        )?
                    };
                    observers.push((
                        format!("{}/{}", channel_group.get_name(), channel.get_name()),
                        observer,
                    ));
                }
            }

            // Now read the data for this data group
            println!("\n    Reading data for Data Group {i}...");
            reader.read_data(data_group)?;
            println!("    Data read successfully.");

            // Process the observers
            println!("\n    Processing Channel Observers for Data Group {i}...");
            for (channel_name, observer) in &observers {
                let nof_samples = observer.get_nof_samples();
                println!("      Observer for \"{channel_name}\": {nof_samples} samples");

                if nof_samples > 0 {
                    // Print first 5 samples
                    println!("        First 5 samples:");
                    for sample_idx in 0..nof_samples.min(5) {
                        let eng_value = observer.get_eng_value(sample_idx);
                        let channel_value = observer.get_channel_value(sample_idx);
                        println!(
                            "          Sample {sample_idx}: Eng: {eng_value:?}, Channel: {channel_value:?}"
                        );
                    }
                }
            }

            // Clear data to free memory
            data_group.clear_data();
            println!("    Cleared data for Data Group {i}.");
        }
    }

    println!("\nSuccessfully read MDF file structure and sample data!");

    Ok(())
}
