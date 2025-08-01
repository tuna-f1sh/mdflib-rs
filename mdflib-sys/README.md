# mdflib-sys

Crate that provides low-level FFI (Foreign Function Interface) bindings to the [`mdflib`](https://github.com/ihedvall/mdflib) C++ library. It is responsible for building, linking and binding the C++ library so that it can be used from Rust.

## Prerequisites

[mdflib](https://github.com/ihedvall/mdflib) has the following dependencies:

* **zlib**: A software library used for data compression.
* **expat**: A stream-oriented XML parser library.
* **cmake** and **build-essential** (or Apple/Windows build tools): Required for building the C++ library from source.

## Adding More mdflib Functions

The bindings are generated using `bindgen` from the manually exposed functions in './src/mdf_c_wrapper.cpp and './src/mdf_c_wrapper.h'. This allows Rust code to call into the C++ library seamlessly.

To add more functions to the bindings, you need to modify the `mdf_c_wrapper.h` and `mdf_c_wrapper.cpp` files to expose the desired C++ functions. `cargo build` will automatically regenerate the bindings when you make changes to these files.

## Build Modes

The `mdflib-sys` crate supports two build modes for linking against the `mdflib` C++ library:

### Using Bundled `mdflib` (Default)

By default, the build script will build and statically link against the bundled `mdflib` source code included in the `bundled` directory. This is the recommended approach for most users as it doesn't require any pre-installation of the C++ library.

To ensure the bundled source is available, ensure that submodules are initialized when cloning the repository:

```bash
git submodule update --init --recursive
```

### Using System `mdflib`

If you have `mdflib` already installed on your system, you can configure the build to use it instead of the bundled version. Use `--no-default-features --features=system` with `cargo`.
