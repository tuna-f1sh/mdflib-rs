//! Example: Reading an MDF file
use mdflib::{MdfReader, Result};
use std::env;

fn main() -> Result<()> {
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

    println!("Opening file...");
    reader.open()?;

    println!("Reading header...");
    reader.read_header()?;
    if let Some(header) = reader.get_header() {
        println!("Header: {header:?}");
    }

    println!("Reading measurement info...");
    reader.read_measurement_info()?;

    println!("Reading metadata...");
    reader.read_everything_but_data()?;
    println!("Number of data groups: {}", reader.get_data_group_count());
    if let Some(data) = reader.get_data_group(0) {
        println!("Data Group 0: {data:?}");
        let description = data.get_description();
        println!("Data Group 0 Description: {description}");
        let channels = data.get_channel_group_count();
        for i in 0..channels {
            if let Some(channel_group) = data.get_channel_group(i) {
                let cg_name = channel_group.get_name();
                let cg_description = channel_group.get_description();
                println!("Channel Group {i}: {cg_name}/{cg_description}");
            }
        }
    }

    println!("Successfully read MDF file structure!");

    Ok(())
}
