# mf4-candump

A CAN message logger that writes to MF4 files, similar to `candump` but outputting to MDF4 format instead of stdout. Uses socketcan-rs for CAN interface access with timestamp support.

## Features

- Logs CAN messages from any SocketCAN interface to MDF4 files
- Auto-generates filenames based on current datetime and CAN interface name
- Supports standard and extended CAN frames
- Proper MDF4 file structure with metadata, file history, and CAN bus configuration
- Graceful shutdown on Ctrl-C or specified duration
- Verbose logging support

## Installation

From the repository root:

```bash
cargo build --bin mf4-candump
```

## Usage

```bash
# Log CAN messages from can0 interface until Ctrl-C
./target/debug/mf4-candump can0

# Log for 60 seconds with custom output file
./target/debug/mf4-candump can0 my_log.mf4 --duration 60

# Enable verbose logging
./target/debug/mf4-candump can0 --verbose
```

### Command Line Options

- `<interface>`: CAN interface to use (e.g., can0, can1, vcan0)
- `<output_file>`: Optional output file name (if not specified, auto-generated)
- `-d, --duration <SECONDS>`: Recording duration in seconds (runs until Ctrl-C if not specified)
- `-f, --filters <FILTERS>`: Optional socket filters in format `ID,MASK` (e.g., `123,FFF` for ID 0x123 with mask 0xFFF)
- `-n, --samples <SAMPLES>`: Number of samples to log (default is unlimited)
- `-H, --hardware-timestamp`: Use hardware timestamps if available (default is software timestamps)
- `-v, --verbose`: Enable verbose logging
- `-h, --help`: Print help information
- `-V, --version`: Print version information

### Auto-generated Filenames

When no output file is specified, filenames are automatically generated in the format:
```
candump_<interface>_<YYYYMMDD_HHMMSS>.mf4
```

For example: `candump_can0_20240731_143022.mf4`

## Output Format

The generated MDF4 files contain:

- **File Header**: Includes author, description, and measurement metadata
- **File History**: Records tool name, version, user, and timestamp
- **CAN Channel Groups**: 
  - `CAN_DataFrame`: Standard CAN data frames
  - `CAN_RemoteFrame`: CAN remote frames
  - `CAN_ErrorFrame`: CAN error frames  
- **Channels**: ID, DLC, data bytes, timestamps, and other CAN frame properties

## Prerequisites

- Linux system with SocketCAN support
- CAN interface configured and up (use `ip link set <interface> up type can bitrate <rate>`)
- Read access to the CAN interface (may require running as root or adding user to appropriate groups)

## Examples

### Basic Usage
```bash
# Bring up a CAN interface (as root)
sudo ip link set can0 up type can bitrate 500000

# Start logging
./target/debug/mf4-candump can0
```

### Testing with Virtual CAN
```bash
# Create virtual CAN interface (as root)
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0

# Start logging in one terminal
./target/debug/mf4-candump vcan0

# Send test messages in another terminal
cansend vcan0 123#DEADBEEF
```
