# mdflib

This crate provides a high-level and safe Rust wrapper around the `mdflib-sys` crate, allowing you to work with MDF (Measurement Data Format) files in an idiomatic Rust way. It abstracts away the `unsafe` FFI calls and provides a clean API for reading and writing MDF files.

## Overview

The `mdflib` crate offers two main entry points for interacting with MDF files:

- `MdfReader`: For reading data from existing MDF files.
- `MdfWriter`: For creating and writing data to new MDF files.

It also provides access to the full MDF file structure, including headers, data groups, channel groups, channels, and more.

It's worth reviewing the [mdflib documentation](https://ihedvall.github.io/mdflib) for a comprehensive understanding of the available features and how to use them.

## Reading an MDF File

Review the docs and code in 'examples/read_mdf.rs'. The example can be run with:

```bash
cargo run --example read_mdf -- <path_to_mdf_file>
```

## Writing an MDF File

Review the docs in 'src/writer.rs' and 'mdflib/tests/test_write.rs'.

See also the 'mf4-candump' example in the repository workspace for a more complete example of writing CAN messages to an MDF file.
