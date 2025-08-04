//! # mf4-candump
//!
//! A CAN message logger that writes to MF4 files, similar to candump but outputting
//! to MDF4 format instead of stdout. Uses socketcan-rs for CAN interface access.
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use clap::Parser;
use mdflib::{writer, CanMessage, MdfBusType};
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook_tokio::Signals;
use socketcan::{CanFilter, CanFrame, CanSocketTimestamp, EmbeddedFrame, Socket, SocketOptions};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use tokio::time::Duration;

/// Command line arguments structure
#[derive(Debug, Parser)]
#[command(name = "mf4-candump")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Logs CAN messages to MF4 files")]
struct Args {
    /// CAN interface to use (e.g., can0)
    interface: String,

    /// Output file path (auto-generated if not specified)
    output: Option<PathBuf>,

    /// CAN filters in format id,mask (can be specified multiple times)
    #[arg(short = 'f', long = "filter", value_name = "ID,MASK")]
    filters: Vec<String>,

    /// Recording duration in seconds (runs until Ctrl-C if not specified)
    #[arg(short = 'd', long = "duration", value_name = "SECONDS")]
    duration: Option<u64>,

    /// Stop after receiving n CAN messages
    #[arg(short = 'n', long = "samples", value_name = "COUNT")]
    samples: Option<u64>,

    /// Enable hardware timestamps (default is software timestamps)
    #[arg(short = 'H', long = "hardware-timestamps")]
    hardware_timestamps: bool,

    /// Metadata to add to the MDF file in format key=value (can be specified multiple times)
    #[arg(short = 'm', long = "metadata", value_name = "KEY=VALUE")]
    metadata: Option<Vec<String>>,

    /// Enable verbose logging
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

/// Parse CAN filter from string format "id,mask"
fn parse_can_filter(filter_str: &str) -> Result<CanFilter> {
    let parts: Vec<&str> = filter_str.split(',').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!(
            "Invalid filter format '{}'. Expected format: id,mask",
            filter_str
        ));
    }

    let id = u32::from_str_radix(parts[0].trim_start_matches("0x"), 16)
        .or_else(|_| parts[0].parse::<u32>())
        .context(format!("Invalid CAN ID '{}'", parts[0]))?;

    let mask = u32::from_str_radix(parts[1].trim_start_matches("0x"), 16)
        .or_else(|_| parts[1].parse::<u32>())
        .context(format!("Invalid CAN mask '{}'", parts[1]))?;

    Ok(CanFilter::new(id, mask))
}

/// Generate an automatic filename based on current datetime and interface
fn generate_filename(interface: &str) -> PathBuf {
    let now: DateTime<Local> = Local::now();
    let filename = format!("candump_{}_{}.mf4", interface, now.format("%Y%m%d_%H%M%S"));
    PathBuf::from(filename)
}

/// Setup MDF writer with proper headers and metadata
fn setup_mdf_writer(
    file_path: &PathBuf,
    interface: &str,
    metadata: &Option<Vec<String>>,
) -> Result<writer::MdfWriter> {
    log::info!("Creating MDF4 file: {}", file_path.display());

    let mut writer = writer::MdfWriter::new(writer::MdfWriterType::MdfBusLogger, file_path)
        .context("Failed to create MDF writer")?;

    // Configure for CAN bus logging
    writer.set_bus_type(MdfBusType::CAN as u16);

    if !writer.create_bus_log_configuration() {
        return Err(anyhow::anyhow!("Failed to create bus log configuration"));
    }

    // Setup header with metadata
    if let Some(mut header) = writer.get_header() {
        header.set_author("mf4-candump");
        header.set_description(&format!("CAN bus log from interface {interface}"));

        // Create file history entry
        if let Some(mut history) = header.create_file_history() {
            history.set_description(&format!("CAN message capture from {interface}"))?;
            history.set_tool_name("mf4-candump")?;
            history.set_tool_version(env!("CARGO_PKG_VERSION"))?;
            history
                .set_user_name(&std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()))?;

            let now: DateTime<Local> = Local::now();
            history.set_time(now.timestamp_nanos_opt().unwrap_or(0) as u64);
        }

        if let Some(meta) = metadata {
            let mut header_meta = header.create_metadata().unwrap();
            for entry in meta {
                let parts: Vec<&str> = entry.splitn(2, '=').collect();
                if parts.len() == 2 {
                    header_meta.set_property_as_string(parts[0], parts[1])?;
                } else {
                    log::warn!("Invalid metadata entry '{entry}', expected format key=value",);
                }
            }
        }
    }

    writer.set_pre_trig_time(0.0);
    writer.set_compress_data(false);

    if !writer.init_measurement() {
        return Err(anyhow::anyhow!("Failed to initialize measurement"));
    }

    Ok(writer)
}

/// Main CAN logging loop
async fn log_can_messages(
    mut writer: writer::MdfWriter,
    interface: &str,
    hardware_timestamps: bool,
    filters: &[CanFilter],
    duration: Option<u64>,
    max_samples: Option<u64>,
    running: Arc<AtomicBool>,
) -> Result<()> {
    log::info!("Opening CAN socket on interface: {interface}");

    let timestamping_mode = if hardware_timestamps {
        socketcan::socket::TimestampingMode::Hardware
    } else {
        socketcan::socket::TimestampingMode::Software
    };

    let addr = socketcan::CanAddr::from_iface(interface)
        .context("Failed to create CAN address from interface")?;
    let socket = CanSocketTimestamp::open_with_timestamping_mode(&addr, timestamping_mode)
        .context("Failed to open CAN socket - is the interface up and accessible?")?;

    // Apply CAN filters if specified
    if !filters.is_empty() {
        log::info!("Applying {} CAN filter(s)", filters.len());
        for (i, _filter) in filters.iter().enumerate() {
            log::debug!("Filter {}: Applied", i + 1);
        }
        socket
            .set_filters(filters)
            .context("Failed to set CAN filters")?;
    }

    // Get the start time in nanoseconds
    let start_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    writer.start_measurement(start_time);

    // Get channel groups for different CAN frame types
    let header = writer.get_header().context("Failed to get header")?;
    let last_dg = header
        .get_last_data_group()
        .context("Failed to get data group")?;

    let can_data_group = last_dg
        .get_channel_group("CAN_DataFrame")
        .context("Failed to get CAN_DataFrame channel group")?;
    let can_error_group = last_dg
        .get_channel_group("CAN_ErrorFrame")
        .context("Failed to get CAN_ErrorFrame channel group")?;
    let can_remote_group = last_dg
        .get_channel_group("CAN_RemoteFrame")
        .context("Failed to get CAN_RemoteFrame channel group")?;

    log::info!("Starting CAN message capture...");
    let mut message_count = 0u64;

    // Create timeout future if duration is specified
    let timeout_future = async {
        if let Some(dur) = duration {
            tokio::time::sleep(Duration::from_secs(dur)).await;
            log::info!("Duration timeout reached");
        } else {
            // If no duration, wait indefinitely
            std::future::pending::<()>().await;
        }
    };

    // Main capture loop
    tokio::select! {
        _ = timeout_future => {
            log::info!("Stopping due to timeout");
        }
        result = async {
            while running.load(Ordering::Relaxed) {
                // Check if we've reached the sample limit
                if let Some(max) = max_samples {
                    if message_count >= max {
                        log::info!("Reached sample limit of {max} messages");
                        break;
                    }
                }

                match socket.read_frame() {
                    Ok((frame, ts)) => {
                        // Convert socketcan frame to mdflib CanMessage
                        let mut can_msg = CanMessage::new();
                        // Extract the raw CAN ID
                        let can_id = match frame.id() {
                            socketcan::Id::Standard(id) => id.as_raw() as u32,
                            socketcan::Id::Extended(id) => id.as_raw(),
                        };
                        can_msg.set_message_id(can_id);
                        can_msg.set_extended_id(frame.is_extended());
                        can_msg.set_dlc(frame.dlc() as u8);
                        can_msg.set_data_bytes(frame.data());
                        let ts = ts.unwrap_or_else(std::time::SystemTime::now);

                        // Save the CAN message to MDF file
                        let nano_secs = ts.duration_since(UNIX_EPOCH).unwrap().as_nanos();
                        match frame {
                            CanFrame::Data(_) => {
                                writer.save_can_message(&can_data_group, nano_secs as u64, &can_msg);
                            }
                            CanFrame::Error(_) => {
                                writer.save_can_message(&can_error_group, nano_secs as u64, &can_msg);
                            }
                            CanFrame::Remote(_) => {
                                writer.save_can_message(&can_remote_group, nano_secs as u64, &can_msg);
                            }
                        }

                        if log::log_enabled!(log::Level::Debug) {
                            let timestamp: f64 = nano_secs as f64 / 1_000_000_000.0; // Convert to seconds
                            log::debug!("Captured CAN message: {timestamp:10.8}, ID={can_id:03X}, DLC={}", frame.dlc());
                        }

                        message_count += 1;
                    }
                    Err(e) => {
                        if e.kind() != socketcan::IoErrorKind::WouldBlock {
                            log::error!("Error reading CAN frame: {e}");
                            // Continue on read errors, but break on persistent errors
                            tokio::time::sleep(Duration::from_millis(10)).await;
                            return Err(e.into());
                        } else {
                            // Allow timeout_future to check on would block
                            tokio::time::sleep(Duration::from_nanos(50)).await;
                        }
                    }
                }
            }
            Ok::<(), anyhow::Error>(())
        } => {
            if let Err(e) = result {
                log::error!("Error in CAN capture loop: {e}");
            }
        }
    }

    let stop_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    log::info!("Captured {message_count} CAN messages total");
    log::info!("Finalizing MDF file...");

    writer.stop_measurement(stop_time);

    if !writer.finalize_measurement() {
        log::warn!("Failed to properly finalize measurement");
    }

    log::info!("MDF file finalized successfully");
    Ok(())
}

/// Setup signal handling for graceful shutdown
async fn setup_signal_handler(running: Arc<AtomicBool>) -> Result<()> {
    use futures::stream::StreamExt;

    let signals = Signals::new([SIGINT, SIGTERM])?;
    let _handle = signals.handle();

    tokio::spawn(async move {
        let mut signals_stream = signals;

        while let Some(signal) = signals_stream.next().await {
            match signal {
                SIGINT | SIGTERM => {
                    log::info!("Received termination signal, shutting down gracefully...");
                    running.store(false, Ordering::Relaxed);
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    let log_level = if args.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Setup mdflib logging
    mdflib::log::set_log_callback_1(Some(mdflib::log::log_callback))
        .context("Failed to setup mdflib logging")?;

    // Parse CAN filters
    let mut can_filters = Vec::new();
    for filter_str in &args.filters {
        match parse_can_filter(filter_str) {
            Ok(filter) => can_filters.push(filter),
            Err(e) => {
                log::error!("Invalid filter '{filter_str}': {e}");
                return Err(e);
            }
        }
    }

    // Determine output file path
    let output_path = args
        .output
        .unwrap_or_else(|| generate_filename(&args.interface));

    log::info!("mf4-candump starting...");
    log::info!("CAN interface: {}", args.interface);
    log::info!("Output file: {}", output_path.display());
    if let Some(duration) = args.duration {
        log::info!("Duration: {duration} seconds");
    } else {
        log::info!("Duration: until Ctrl-C");
    }
    if let Some(samples) = args.samples {
        log::info!("Sample limit: {samples} messages");
    }
    if !can_filters.is_empty() {
        log::info!("CAN filters: {} active", can_filters.len());
    }

    // Setup signal handling
    let running = Arc::new(AtomicBool::new(true));
    setup_signal_handler(running.clone()).await?;

    // Setup MDF writer
    let writer = setup_mdf_writer(&output_path, &args.interface, &args.metadata)?;

    // Start logging
    match log_can_messages(
        writer,
        &args.interface,
        args.hardware_timestamps,
        &can_filters,
        args.duration,
        args.samples,
        running,
    )
    .await
    {
        Ok(()) => {
            log::info!("CAN logging completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("CAN logging failed: {e}");
            Err(e)
        }
    }
}
