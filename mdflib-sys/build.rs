use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Handle linking strategy based on features
    if cfg!(feature = "bundled") {
        build_bundled(&out_dir, &manifest_dir);
    } else if cfg!(feature = "system") {
        setup_system_linking();
    } else {
        panic!("Either 'bundled' or 'system' feature must be enabled");
    }

    // Generate bindings - this should always work
    generate_bindings(&manifest_dir, &out_dir);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=bundled");
    println!("cargo:rerun-if-changed=src/mdf_c_wrapper.h");
    println!("cargo:rerun-if-changed=src/mdf_c_wrapper.cpp");

    println!(
        "cargo:warning=TARGET: {}",
        env::var("TARGET").unwrap_or_else(|_| "unknown".to_string())
    );
}

fn build_bundled(out_dir: &Path, manifest_dir: &Path) {
    let bundled_dir = manifest_dir.join("bundled");
    let build_dir = out_dir.join("build");
    let install_dir = out_dir.join("install");

    // Create build directory
    std::fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    std::fs::create_dir_all(&install_dir).expect("Failed to create install directory");

    // Check if we have the mdflib source
    if !bundled_dir.exists() {
        panic!(
            "Bundled mdflib source not found at {}. \
            Please run: git submodule update --init --recursive \
            or download mdflib source to the bundled/ directory",
            bundled_dir.display()
        );
    }

    // Setup dependencies first
    setup_dependencies();

    // Configure with CMake - enable the export library
    let mut cmake_config = Command::new("cmake");
    cmake_config
        .current_dir(&build_dir)
        .arg(&bundled_dir)
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", install_dir.display()))
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DBUILD_SHARED_LIBS=OFF") // Build shared library
        .arg("-DMDF_BUILD_SHARED_LIB=OFF") // Build the export library
        .arg("-DMDF_BUILD_SHARED_LIB_NET=OFF") // Don't build .NET wrapper
        .arg("-DMDF_BUILD_TEST=OFF") // Don't build tests
        .arg("-DMDF_BUILD_DOC=OFF") // Don't build documentation
        .arg("-DMDF_BUILD_TOOLS=OFF"); // Don't build tools

    // Platform-specific CMake settings
    if cfg!(target_os = "windows") {
        if cfg!(target_env = "msvc") {
            cmake_config.arg("-G").arg("Visual Studio 16 2019");
            if cfg!(target_arch = "x86_64") {
                cmake_config.arg("-A").arg("x64");
            } else if cfg!(target_arch = "x86") {
                cmake_config.arg("-A").arg("Win32");
            }
        }
    } else {
        cmake_config.arg("-G").arg("Unix Makefiles");
    }

    // Add C++ standard
    cmake_config.arg("-DCMAKE_CXX_STANDARD=17");

    // Help CMake find dependencies
    add_dependency_hints(&mut cmake_config);

    println!("Running CMake configure...");
    let cmake_output = cmake_config
        .output()
        .expect("Failed to run CMake configure");

    if !cmake_output.status.success() {
        eprintln!("CMake configure failed:");
        eprintln!("stdout: {}", String::from_utf8_lossy(&cmake_output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&cmake_output.stderr));

        // Provide helpful error messages
        let stderr = String::from_utf8_lossy(&cmake_output.stderr);
        if stderr.contains("Could NOT find ZLIB") {
            eprintln!("\nzlib not found. Please install zlib development libraries:");
            print_zlib_install_instructions();
        }
        if stderr.contains("Could NOT find EXPAT") {
            eprintln!("\nexpat not found. Please install expat development libraries:");
            print_expat_install_instructions();
        }

        panic!("CMake configuration failed");
    }

    // Build with CMake
    println!("Building mdflib...");
    let mut cmake_build = Command::new("cmake");
    cmake_build
        .current_dir(&build_dir)
        .arg("--build")
        .arg(".")
        .arg("--config")
        .arg("Release")
        .arg("--target")
        .arg("install");

    // Use parallel build if possible
    if let Ok(jobs) = env::var("NUM_JOBS") {
        cmake_build.arg("--parallel").arg(jobs);
    } else {
        // Default to number of CPUs
        if let Ok(jobs) = std::thread::available_parallelism() {
            cmake_build.arg("--parallel").arg(jobs.get().to_string());
        }
    }

    let build_output = cmake_build.output().expect("Failed to run CMake build");

    if !build_output.status.success() {
        panic!(
            "CMake build failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&build_output.stdout),
            String::from_utf8_lossy(&build_output.stderr)
        );
    }

    // Build the C wrapper
    let mut cc_build = cc::Build::new();
    cc_build
        .cpp(true)
        .file("src/mdf_c_wrapper.cpp")
        .include(install_dir.join("include"))
        .include(bundled_dir.join("include"))
        .flag("-Wno-overloaded-virtual") // Suppress mdf::MdString::ToXml' hides overloaded virtual function
        .flag("-std=c++17");

    if cfg!(target_os = "macos") {
        cc_build.cpp_link_stdlib("c++");
    } else {
        cc_build.cpp_link_stdlib("stdc++");
    }
    cc_build.compile("mdf_c_wrapper");

    // Set up linking
    setup_bundled_linking(&install_dir);
}

fn setup_dependencies() {
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-changed=ZLIB_LIBRARY");
    println!("cargo:rerun-if-changed=ZLIB_INCLUDE_DIR");
    println!("cargo:rerun-if-changed=EXPAT_LIBRARY");
    println!("cargo:rerun-if-changed=EXPAT_INCLUDE_DIR");

    // Try to find zlib
    setup_zlib_dependency();

    // Try to find expat
    setup_expat_dependency();
}

fn setup_zlib_dependency() {
    // First try pkg-config
    if let Ok(zlib) = pkg_config::probe_library("zlib") {
        println!("Found zlib via pkg-config");
        for path in zlib.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for lib in zlib.libs {
            println!("cargo:rustc-link-lib={lib}");
        }
        return;
    }

    // Then try environment variables
    if let Ok(zlib_lib) = env::var("ZLIB_LIBRARY") {
        println!("Found zlib via ZLIB_LIBRARY environment variable");

        // Extract directory and library name from full path
        let lib_path = PathBuf::from(&zlib_lib);
        if let Some(lib_dir) = lib_path.parent() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
        }

        // Extract library name (remove lib prefix and extension)
        if let Some(lib_name) = lib_path.file_stem() {
            let lib_name_str = lib_name.to_string_lossy();
            let clean_name = lib_name_str.strip_prefix("lib").unwrap_or(&lib_name_str);
            println!("cargo:rustc-link-lib={clean_name}");
        }

        // Add include directory if provided
        if let Ok(zlib_include) = env::var("ZLIB_INCLUDE_DIR") {
            println!("cargo:include={zlib_include}");
        }
        return;
    }

    // Finally fallback to default behavior
    println!("cargo:warning=zlib not found via pkg-config or environment variables, using system defaults");
    link_zlib_fallback();
}

fn setup_expat_dependency() {
    // First try pkg-config
    if let Ok(expat) = pkg_config::probe_library("expat") {
        println!("Found expat via pkg-config");
        for path in expat.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for lib in expat.libs {
            println!("cargo:rustc-link-lib={lib}");
        }
        return;
    }

    // Then try environment variables
    if let Ok(expat_lib) = env::var("EXPAT_LIBRARY") {
        println!("Found expat via EXPAT_LIBRARY environment variable");

        // Extract directory and library name from full path
        let lib_path = PathBuf::from(&expat_lib);
        if let Some(lib_dir) = lib_path.parent() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
        }

        // Extract library name (remove lib prefix and extension)
        if let Some(lib_name) = lib_path.file_stem() {
            let lib_name_str = lib_name.to_string_lossy();
            let clean_name = lib_name_str.strip_prefix("lib").unwrap_or(&lib_name_str);
            println!("cargo:rustc-link-lib={clean_name}");
        }

        // Add include directory if provided
        if let Ok(expat_include) = env::var("EXPAT_INCLUDE_DIR") {
            println!("cargo:include={expat_include}");
        }
        return;
    }

    // Finally fallback to default behavior
    println!("cargo:warning=expat not found via pkg-config or environment variables, using system defaults");
    link_expat_fallback();
}

fn link_zlib_fallback() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=zlib");
    } else {
        println!("cargo:rustc-link-lib=z");
    }
}

fn link_expat_fallback() {
    println!("cargo:rustc-link-lib=expat");
}

fn add_dependency_hints(cmake_config: &mut Command) {
    // Check for environment variable hints first
    if let Ok(zlib_lib) = env::var("ZLIB_LIBRARY") {
        let lib_path = PathBuf::from(&zlib_lib);
        if let Some(lib_dir) = lib_path.parent() {
            if let Some(root_dir) = lib_dir.parent() {
                cmake_config.arg(format!("-DZLIB_ROOT={}", root_dir.display()));
            }
        }
        cmake_config.arg(format!("-DZLIB_LIBRARY={zlib_lib}"));
    }

    if let Ok(zlib_include) = env::var("ZLIB_INCLUDE_DIR") {
        cmake_config.arg(format!("-DZLIB_INCLUDE_DIR={zlib_include}"));
    }

    if let Ok(expat_lib) = env::var("EXPAT_LIBRARY") {
        let lib_path = PathBuf::from(&expat_lib);
        if let Some(lib_dir) = lib_path.parent() {
            if let Some(root_dir) = lib_dir.parent() {
                cmake_config.arg(format!("-DEXPAT_ROOT={}", root_dir.display()));
            }
        }
        cmake_config.arg(format!("-DEXPAT_LIBRARY={expat_lib}"));
    }

    if let Ok(expat_include) = env::var("EXPAT_INCLUDE_DIR") {
        cmake_config.arg(format!("-I{expat_include}"));
    }

    // If no environment variables, fall back to platform-specific discovery
    if env::var("ZLIB_LIBRARY").is_err() && env::var("EXPAT_LIBRARY").is_err() {
        add_platform_dependency_hints(cmake_config);
    }
}

fn add_platform_dependency_hints(cmake_config: &mut Command) {
    if cfg!(target_os = "macos") {
        // Homebrew locations
        if std::path::Path::new("/opt/homebrew/opt/zlib").exists() {
            cmake_config.arg("-DZLIB_ROOT=/opt/homebrew/opt");
            cmake_config.arg("-DZLIB_LIBRARY=/opt/homebrew/opt/zlib/lib/libz.a");
        }
        if std::path::Path::new("/opt/homebrew/opt/expat").exists() {
            cmake_config.arg("-DEXPAT_ROOT=/opt/homebrew/opt");
            cmake_config.arg("-DEXPAT_LIBRARY=/opt/homebrew/opt/expat/lib/libexpat.a");
        }

        // Also try system location
        if std::path::Path::new("/usr/include/zlib.h").exists() {
            cmake_config.arg("-DZLIB_ROOT=/usr");
        }
        if std::path::Path::new("/usr/include/expat.h").exists() {
            cmake_config.arg("-DEXPAT_ROOT=/usr");
        }
    }
}

fn setup_bundled_linking(install_dir: &Path) {
    let lib_dir = install_dir.join("lib");
    let lib64_dir = install_dir.join("lib64");

    // Add library search paths
    if lib_dir.exists() {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
    }
    if lib64_dir.exists() {
        println!("cargo:rustc-link-search=native={}", lib64_dir.display());
    }

    // Link the mdflib core library (static)
    println!("cargo:rustc-link-lib=static=mdf");

    // Link the C wrapper library (static)
    println!("cargo:rustc-link-lib=static=mdf_c_wrapper");

    // Link required dependencies
    // Removed direct calls to setup_zlib_dependency and setup_expat_dependency
    // as they are now called from setup_dependencies
    link_dependencies();

    // Platform-specific system library dependencies
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=kernel32");
        println!("cargo:rustc-link-lib=dylib=ws2_32");
        println!("cargo:rustc-link-lib=dylib=advapi2");
        println!("cargo:rustc-link-lib=dylib=shell32");
        println!("cargo:rustc-link-lib=dylib=ole32");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-lib=dylib=pthread");
        println!("cargo:rustc-link-lib=dylib=dl");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++"); // Use libc++ on macOS
        println!("cargo:rustc-link-lib=dylib=System");
        println!("cargo:rustc-link-lib=framework=Foundation");
    }

    // Add include directory for bindgen
    let include_dir = install_dir.join("include");
    if include_dir.exists() {
        println!("cargo:include={}", include_dir.display());
    }
}

fn link_dependencies() {
    // Link zlib (required by mdflib)
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=zlib");
    } else {
        println!("cargo:rustc-link-lib=z");
    }

    // Link expat (required by mdflib for XML parsing)
    println!("cargo:rustc-link-lib=expat");
}

fn setup_system_linking() {
    // Build the C wrapper
    let mut cc_build = cc::Build::new();
    cc_build
        .cpp(true)
        .file("src/mdf_c_wrapper.cpp")
        .flag("-Wno-overloaded-virtual") // Suppress mdf::MdString::ToXml' hides overloaded virtual function
        .flag("-std=c++17");

    if cfg!(target_os = "macos") {
        cc_build.cpp_link_stdlib("c++");
    } else {
        cc_build.cpp_link_stdlib("stdc++");
    }

    // Try to find system-installed mdflib using pkg-config
    if let Ok(library) = pkg_config::Config::new()
        .atleast_version("2.1")
        .probe("mdflib")
    {
        for path in library.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for lib in library.libs {
            println!("cargo:rustc-link-lib={lib}");
        }

        // Set include paths for bindgen
        for path in library.include_paths {
            println!("cargo:include={}", path.display());
        }
    } else {
        println!("cargo:warning=pkg-config failed, trying manual discovery");

        // Fallback: assume library is in standard locations
        println!("cargo:rustc-link-lib=mdf");

        // Also link dependencies since mdflib depends on them
        link_dependencies();

        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-lib=dylib=stdc++"); // Explicitly link C++ standard library
            println!("cargo:rustc-link-lib=dylib=m");
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=dl");
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-search=native=/usr/lib");
            println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
            println!("cargo:include=/usr/local/include");
            println!("cargo:include=/usr/include");
            cc_build.include("/usr/local/include");
            cc_build.include("/usr/include");
        } else if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-lib=dylib=c++"); // Explicitly link C++ standard library
            println!("cargo:rustc-link-lib=dylib=System");
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
            println!("cargo:include=/usr/local/include");
            println!("cargo:include=/opt/homebrew/include");
            cc_build.include("/usr/local/include");
            cc_build.include("/opt/homebrew/include");
        } else if cfg!(target_os = "windows") {
            // Windows-specific paths
            println!("cargo:rustc-link-lib=dylib=stdc++"); // Explicitly link C++ standard library
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=kernel32");
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=advapi2");
            println!("cargo:rustc-link-lib=dylib=shell32");
            println!("cargo:rustc-link-lib=dylib=ole32");
            println!("cargo:rustc-link-search=native=C:/Program Files/mdflib/lib");
            println!("cargo:include=C:/Program Files/mdflib/include");
            cc_build.include("C:/Program Files/mdflib/include");
        }
    }

    cc_build.compile("mdf_c_wrapper");
}

fn generate_bindings(manifest_dir: &Path, out_dir: &Path) {
    let wrapper_path = manifest_dir.join("src").join("mdf_c_wrapper.h");

    println!("Generating bindings from {}", wrapper_path.display());

    let mut bindgen_builder = bindgen::Builder::default()
        .header(wrapper_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-xc++")
        .clang_arg("-std=c++17");

    // Add bundled include path
    let bundled_include = manifest_dir.join("bundled").join("include");
    if bundled_include.exists() {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", bundled_include.display()));
    }

    // Add system include paths for dependencies
    if cfg!(target_os = "macos") {
        // Add Homebrew paths if they exist
        if std::path::Path::new("/opt/homebrew/include").exists() {
            bindgen_builder = bindgen_builder.clang_arg("-I/opt/homebrew/include");
        }

        // Try to find the system SDK
        if let Ok(output) = Command::new("xcrun").args(["--show-sdk-path"]).output() {
            if output.status.success() {
                let sdk_path = String::from_utf8_lossy(&output.stdout);
                bindgen_builder =
                    bindgen_builder.clang_arg(format!("-I{}/usr/include", sdk_path.trim()));
            }
        }
    }

    // Add dependency include paths
    if let Ok(zlib_include) = env::var("ZLIB_INCLUDE_DIR") {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{zlib_include}"));
    }

    if let Ok(expat_include) = env::var("EXPAT_INCLUDE_DIR") {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{expat_include}"));
    }

    let bindings = bindgen_builder
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Configure enum handling to avoid name conflicts
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .blocklist_type("std::.*") // Block all std types
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .derive_partialeq(true)
        .derive_partialord(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("Successfully generated bindings");
}

fn print_zlib_install_instructions() {
    if cfg!(target_os = "linux") {
        eprintln!("  Ubuntu/Debian: sudo apt install zlib1g-dev");
        eprintln!("  CentOS/RHEL/Fedora: sudo dnf install zlib-devel");
    }
    eprintln!("\nAlternatively, set environment variables:");
    eprintln!("  ZLIB_LIBRARY=/path/to/libz.so (or .a/.lib)");
    eprintln!("  ZLIB_INCLUDE_DIR=/path/to/zlib/headers");
}

fn print_expat_install_instructions() {
    if cfg!(target_os = "linux") {
        eprintln!("  Ubuntu/Debian: sudo apt install libexpat1-dev");
        eprintln!("  CentOS/RHEL/Fedora: sudo dnf install expat-devel");
    }
    eprintln!("\nAlternatively, set environment variables:");
    eprintln!("  EXPAT_LIBRARY=/path/to/libexpat.so (or .a/.lib)");
    eprintln!("  EXPAT_INCLUDE_DIR=/path/to/expat/headers");
}
