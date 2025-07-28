# MDFlib Rust Wrapper

A crate that provides Rust bindings to the [`mdflib`](https://github.com/ihedvall/mdflib) C++ library, enabling you to read and write MDF (Measurement Data Format) files.

## Why another MDF library?

While there are other MDF libraries available for Rust, such as `asammdf` and `rsmdf`, they are either incomplete or lack certain features like write support or bus logging helpers. This library aims to provide a more comprehensive solution by wrapping the feature-rich `mdflib` C++ library.

This project is intended as a proof of concept and a quick way to get a project that depends on MDF logging up and running. In the long term, the goal is to contribute to the existing Rust MDF libraries and reduce the reliance on the C++ `mdflib`.

## Prerequisites

[mdflib](https://github.com/ihedvall/mdflib) has the following dependencies:

* **zlib**: A software library used for data compression.
* **expat**: A stream-oriented XML parser library.
* **cmake** and **build-essential** (or Apple/Windows build tools): Required for building the C++ library from source.

You can install these dependencies using your system's package manager.

## Usage

Review the [mdflib documentation](https://ihedvall.github.io/mdflib/), the 'examples/' and 'tests/' directories in this repository.

**Note:** The current implementation is mostly `unsafe` due to the nature of FFI (Foreign Function Interface) with C++.

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
