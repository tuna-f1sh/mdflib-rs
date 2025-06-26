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
    println!("Opening MDF file: {}", filename);

    let mut reader = MdfReader::new(filename)?;

    if !reader.is_ok() {
        eprintln!("Reader is not in a valid state");
        return Ok(());
    }

    println!("Opening file...");
    reader.open()?;

    println!("Reading header...");
    reader.read_header()?;

    println!("Reading measurement info...");
    reader.read_measurement_info()?;

    println!("Reading metadata...");
    reader.read_everything_but_data()?;

    println!("Successfully read MDF file structure!");

    Ok(())
}
