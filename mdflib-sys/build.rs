use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Handle linking strategy based on features
    if cfg!(feature = "bundled") {
        build_bundled();
    } else if cfg!(feature = "system") {
        link_system_library();
    } else {
        panic!("Either 'bundled' or 'system' feature must be enabled");
    }

    // Generate bindings
    generate_bindings();

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
}

fn build_bundled() {
    println!("cargo:rustc-link-lib=static=mdflib");

    // This would compile mdflib from source
    // For now, we'll assume the library is available
    // In a real implementation, you'd either:
    // 1. Include mdflib source as a git submodule
    // 2. Download and compile it in the build script
    // 3. Use CMake to build the existing CMakeLists.txt

    let mut build = cc::Build::new();
    build.cpp(true).std("c++17").warnings(false);

    // Add include directories
    build.include("bundled/include");

    // Platform-specific settings
    if cfg!(target_os = "windows") {
        build.define("_WIN32", None);
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=kernel32");
    } else if cfg!(target_os = "linux") {
        build.define("__linux__", None);
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if cfg!(target_os = "macos") {
        build.define("__APPLE__", None);
        println!("cargo:rustc-link-lib=dylib=c++");
    }

    // Compile the export wrapper if we have the source
    // build.file("bundled/src/MdfExport.cpp");
    // build.compile("mdflib");

    println!("cargo:warning=Bundled compilation not fully implemented yet. Please use system feature for now.");
}

fn link_system_library() {
    // Try to find system-installed mdflib using pkg-config
    if let Ok(library) = pkg_config::probe_library("mdflib") {
        // pkg-config found the library
        for path in library.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for lib in library.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    } else {
        // Fallback: assume library is in standard locations
        println!("cargo:rustc-link-lib=mdflib");

        // You might want to add common library search paths
        if cfg!(target_os = "windows") {
            // Windows-specific paths
        } else if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-search=native=/usr/lib");
        } else if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
        }
    }
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Configure bindgen for C++ compatibility
        .allowlist_function("MdfReader.*")
        .allowlist_function("MdfWriter.*")
        .allowlist_function("MdfFile.*")
        .allowlist_function("MdfHeader.*")
        .allowlist_function("MdfDataGroup.*")
        .allowlist_function("MdfChannel.*")
        .allowlist_function("MdfChannelGroup.*")
        .allowlist_function("MdfChannelObserver.*")
        .allowlist_function("CanMessage.*")
        .allowlist_type("MdfWriterType")
        .allowlist_type("ChannelType")
        .allowlist_type("ChannelDataType")
        .allowlist_type("ConversionType")
        .allowlist_type("CanErrorType")
        .allowlist_var(".*")
        // Generate Debug traits
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .derive_partialeq(true)
        .derive_partialord(true)
        // Handle C++ namespaces and naming
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
