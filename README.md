# MDFlib Rust Wrapper

A workspace that provides Rust bindings and wrapper code to the [`mdflib`](https://github.com/ihedvall/mdflib) C++ library, enabling you to read and write MDF (Measurement Data Format) files.

## Why not native Rust?

While there are MDF libraries available for Rust: [`danielrisca/asammdf`](https://github.com/danielhrisca/asammdf), [`rsmdf`](https://github.com/Liberty009/rsmdf) [`H202-IO/asammdf`](https://github.com/H2O2-IO/asammdf) (looked most viable for development to me), they are either incomplete or lack certain features like write support or bus logging helpers. This library aims to provide a more comprehensive solution by wrapping the feature-rich `mdflib` C++ library.

This project is intended as a proof of concept and a quick way to get a project that depends on MDF logging up and running. In the long term, the goal is to contribute to the existing Rust MDF libraries and reduce the reliance on the C++ `mdflib`.

It was also an experiement for me using bindgen and Copilot agent to write some of the wrapper code.

## Prerequisites

[mdflib](https://github.com/ihedvall/mdflib) has the following dependencies:

* **zlib**: A software library used for data compression.
* **expat**: A stream-oriented XML parser library.
* **cmake** and **build-essential** (or Apple/Windows build tools): Required for building the C++ library from source.

You can install these dependencies using your system's package manager.

## Usage

Review the [mdflib documentation](https://ihedvall.github.io/mdflib/), the 'mdflib/examples/' and 'mdflib/tests/' directories in this repository.

> [!WARNING]
> The current implementation is mostly `unsafe` due to the nature of FFI (Foreign Function Interface) with C++. There are also some lifetime privledges taken to allow for full access to the C++ API. Use with caution and ensure you understand the implications of using `unsafe` code in Rust.

### CAN Message Logging with mf4-candump

The workspace includes `mf4-candump`, a CAN message logger that writes to MF4 files, similar to `candump` but outputting to MDF4 format instead of stdout. It's an example of how to use the `mdflib` Rust bindings to log CAN messages.

```bash
# Log CAN messages from can0 interface
cargo run --bin mf4-candump -p mf4-candump can0
```

See the [mf4-candump README](mf4-candump/README.md) for detailed usage instructions.

## Building from Source

If you want to build the library from source, you'll need to have the `mdflib` C++ library available on your system. You can either install it system-wide or use the bundled version.

### Using Bundled `mdflib`

By default, the build script will attempt to build and link against the bundled `mdflib` source code. This is the recommended approach for most users. Ensure that the repository is cloned with submodules initialized:

```bash
git clone --recurse-submodules
```

### Using System `mdflib`

If you have `mdflib` installed on your system, you can configure the build to use it instead of the bundled version. This can be useful if you want to use a specific version of `mdflib` or if you have a system-wide installation that you want to link against.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.
