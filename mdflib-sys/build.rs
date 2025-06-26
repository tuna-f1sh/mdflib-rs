use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Handle linking strategy based on features
    if cfg!(feature = "bundled") {
        build_bundled(&out_dir, &manifest_dir);
    } else if cfg!(feature = "system") {
        link_system_library();
    } else {
        panic!("Either 'bundled' or 'system' feature must be enabled");
    }

    // Generate bindings
    generate_bindings(&manifest_dir);

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=bundled");
}

fn build_bundled(out_dir: &PathBuf, manifest_dir: &PathBuf) {
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

    // Configure with CMake
    let mut cmake_config = Command::new("cmake");
    cmake_config
        .current_dir(&build_dir)
        .arg(&bundled_dir)
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", install_dir.display()))
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DBUILD_SHARED_LIBS=OFF") // Build static library
        .arg("-DMDF_BUILD_TEST=OFF") // Don't build tests
        .arg("-DMDF_BUILD_DOC=OFF") // Don't build documentation
        .arg("-DMDF_BUILD_TOOLS=OFF") // Don't build tools
        .arg("-DMDF_BUILD_SHARED_LIB=OFF") // Build static only
        .arg("-DMDF_BUILD_SHARED_LIB_NET=OFF"); // Don't build .NET wrapper

    // Platform-specific CMake settings
    if cfg!(target_os = "windows") {
        if cfg!(target_env = "msvc") {
            cmake_config.arg("-G").arg("Visual Studio 16 2019");
            if cfg!(target_arch = "x86_64") {
                cmake_config.arg("-A").arg("x64");
            } else if cfg!(target_arch = "x86") {
                cmake_config.arg("-A").arg("Win32");
            }
        } else {
            cmake_config.arg("-G").arg("MinGW Makefiles");
        }

        // Help CMake find dependencies on Windows
        if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
            cmake_config.arg(format!(
                "-DCMAKE_TOOLCHAIN_FILE={}/scripts/buildsystems/vcpkg.cmake",
                vcpkg_root
            ));
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

    // Set up linking
    setup_bundled_linking(&install_dir);
}

fn setup_dependencies() {
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=ZLIB_LIBRARY");
    println!("cargo:rerun-if-env-changed=ZLIB_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=EXPAT_LIBRARY");
    println!("cargo:rerun-if-env-changed=EXPAT_INCLUDE_DIR");

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
            println!("cargo:rustc-link-lib={}", lib);
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
            let clean_name = if lib_name_str.starts_with("lib") {
                &lib_name_str[3..]
            } else {
                &lib_name_str
            };
            println!("cargo:rustc-link-lib={}", clean_name);
        } else {
            // Fallback if we can't parse the path
            if cfg!(target_os = "windows") {
                println!("cargo:rustc-link-lib=zlib");
            } else {
                println!("cargo:rustc-link-lib=z");
            }
        }

        // Add include directory if provided
        if let Ok(zlib_include) = env::var("ZLIB_INCLUDE_DIR") {
            println!("cargo:include={}", zlib_include);
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
            println!("cargo:rustc-link-lib={}", lib);
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
            let clean_name = if lib_name_str.starts_with("lib") {
                &lib_name_str[3..]
            } else {
                &lib_name_str
            };
            println!("cargo:rustc-link-lib={}", clean_name);
        } else {
            // Fallback if we can't parse the path
            println!("cargo:rustc-link-lib=expat");
        }

        // Add include directory if provided
        if let Ok(expat_include) = env::var("EXPAT_INCLUDE_DIR") {
            println!("cargo:include={}", expat_include);
        }
        return;
    }

    // Finally fallback to default behavior
    println!("cargo:warning=expat not found via pkg-config or environment variables, using system defaults");
    link_expat_fallback();
}

fn link_zlib_fallback() {
    if cfg!(target_os = "windows") {
        // On Windows, we might need to help find zlib
        if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
            let vcpkg_lib = format!("{}/installed/x64-windows/lib", vcpkg_root);
            println!("cargo:rustc-link-search=native={}", vcpkg_lib);
        }
        println!("cargo:rustc-link-lib=zlib");
    } else {
        println!("cargo:rustc-link-lib=z");
    }
}

fn link_expat_fallback() {
    if cfg!(target_os = "windows") {
        // On Windows with vcpkg
        if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
            let vcpkg_lib = format!("{}/installed/x64-windows/lib", vcpkg_root);
            println!("cargo:rustc-link-search=native={}", vcpkg_lib);
        }
        println!("cargo:rustc-link-lib=expat");
    } else {
        println!("cargo:rustc-link-lib=expat");
    }
}

fn add_dependency_hints(cmake_config: &mut Command) {
    // Help CMake find zlib and expat using environment variables first

    // Check for environment variable hints first
    if let Ok(zlib_lib) = env::var("ZLIB_LIBRARY") {
        let lib_path = PathBuf::from(&zlib_lib);
        if let Some(lib_dir) = lib_path.parent() {
            if let Some(root_dir) = lib_dir.parent() {
                cmake_config.arg(format!("-DZLIB_ROOT={}", root_dir.display()));
            }
        }
        cmake_config.arg(format!("-DZLIB_LIBRARY={}", zlib_lib));
    }

    if let Ok(zlib_include) = env::var("ZLIB_INCLUDE_DIR") {
        cmake_config.arg(format!("-DZLIB_INCLUDE_DIR={}", zlib_include));
    }

    if let Ok(expat_lib) = env::var("EXPAT_LIBRARY") {
        let lib_path = PathBuf::from(&expat_lib);
        if let Some(lib_dir) = lib_path.parent() {
            if let Some(root_dir) = lib_dir.parent() {
                cmake_config.arg(format!("-DEXPAT_ROOT={}", root_dir.display()));
            }
        }
        cmake_config.arg(format!("-DEXPAT_LIBRARY={}", expat_lib));
    }

    if let Ok(expat_include) = env::var("EXPAT_INCLUDE_DIR") {
        cmake_config.arg(format!("-DEXPAT_INCLUDE_DIR={}", expat_include));
    }

    // If no environment variables, fall back to platform-specific discovery
    if env::var("ZLIB_LIBRARY").is_err() && env::var("EXPAT_LIBRARY").is_err() {
        add_platform_dependency_hints(cmake_config);
    }

    // Force CMake to find required versions
    cmake_config.arg("-DZLIB_FIND_REQUIRED=ON");
    cmake_config.arg("-DEXPAT_FIND_REQUIRED=ON");
}

fn add_platform_dependency_hints(cmake_config: &mut Command) {
    // Help CMake find zlib and expat in standard locations

    if cfg!(target_os = "linux") {
        // Common locations on Linux
        cmake_config.arg("-DZLIB_ROOT=/usr");
        cmake_config.arg("-DEXPAT_ROOT=/usr");

        // Also check for manually installed versions
        if std::path::Path::new("/usr/local/include/zlib.h").exists() {
            cmake_config.arg("-DZLIB_ROOT=/usr/local");
        }
        if std::path::Path::new("/usr/local/include/expat.h").exists() {
            cmake_config.arg("-DEXPAT_ROOT=/usr/local");
        }
    } else if cfg!(target_os = "macos") {
        // macOS locations
        cmake_config.arg("-DZLIB_ROOT=/usr/local");
        cmake_config.arg("-DEXPAT_ROOT=/usr/local");

        // Homebrew locations
        if std::path::Path::new("/opt/homebrew/include/zlib.h").exists() {
            cmake_config.arg("-DZLIB_ROOT=/opt/homebrew");
        }
        if std::path::Path::new("/opt/homebrew/include/expat.h").exists() {
            cmake_config.arg("-DEXPAT_ROOT=/opt/homebrew");
        }

        // Also try system location
        if std::path::Path::new("/usr/include/zlib.h").exists() {
            cmake_config.arg("-DZLIB_ROOT=/usr");
        }
        if std::path::Path::new("/usr/include/expat.h").exists() {
            cmake_config.arg("-DEXPAT_ROOT=/usr");
        }
    } else if cfg!(target_os = "windows") {
        // Windows with vcpkg
        if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
            let vcpkg_installed = format!("{}/installed/x64-windows", vcpkg_root);
            cmake_config.arg(format!("-DZLIB_ROOT={}", vcpkg_installed));
            cmake_config.arg(format!("-DEXPAT_ROOT={}", vcpkg_installed));
        }

        // Try to find libraries in common locations
        let common_paths = [
            "C:/vcpkg/installed/x64-windows",
            "C:/Program Files/zlib",
            "C:/Program Files/expat",
            "C:/zlib",
            "C:/expat",
        ];

        for path in &common_paths {
            if std::path::Path::new(&format!("{}/include/zlib.h", path)).exists() {
                cmake_config.arg(format!("-DZLIB_ROOT={}", path));
            }
            if std::path::Path::new(&format!("{}/include/expat.h", path)).exists() {
                cmake_config.arg(format!("-DEXPAT_ROOT={}", path));
            }
        }
    }
}

fn setup_bundled_linking(install_dir: &PathBuf) {
    let lib_dir = install_dir.join("lib");
    let lib64_dir = install_dir.join("lib64");

    // Add library search paths
    if lib_dir.exists() {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
    }
    if lib64_dir.exists() {
        println!("cargo:rustc-link-search=native={}", lib64_dir.display());
    }

    // Link the main mdflib library
    println!("cargo:rustc-link-lib=static=mdf");

    // Link required dependencies
    link_dependencies();

    // Platform-specific system library dependencies
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=kernel32");
        println!("cargo:rustc-link-lib=dylib=ws2_32");
        println!("cargo:rustc-link-lib=dylib=advapi32");
        println!("cargo:rustc-link-lib=dylib=shell32");
        println!("cargo:rustc-link-lib=dylib=ole32");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-lib=dylib=pthread");
        println!("cargo:rustc-link-lib=dylib=dl");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
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

fn link_system_library() {
    // Try to find system-installed mdflib using pkg-config
    if let Ok(library) = pkg_config::Config::new()
        .atleast_version("4.0")
        .probe("mdflib")
    {
        for path in library.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for lib in library.libs {
            println!("cargo:rustc-link-lib={}", lib);
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
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-search=native=/usr/lib");
            println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
            println!("cargo:include=/usr/local/include");
            println!("cargo:include=/usr/include");
        } else if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
            println!("cargo:include=/usr/local/include");
            println!("cargo:include=/opt/homebrew/include");
        } else if cfg!(target_os = "windows") {
            // Windows-specific paths
            println!("cargo:rustc-link-search=native=C:/Program Files/mdflib/lib");
            println!("cargo:include=C:/Program Files/mdflib/include");
        }
    }
}

fn generate_bindings(manifest_dir: &PathBuf) {
    let wrapper_h = manifest_dir.join("wrapper.h");

    let mut bindgen_builder = bindgen::Builder::default()
        .header(wrapper_h.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-std=c++17");

    // Add include paths from environment
    if let Ok(include_paths) = env::var("DEP_MDF_INCLUDE") {
        for path in include_paths.split(';') {
            if !path.is_empty() {
                bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", path));
            }
        }
    }

    // Add include paths from environment variables
    if let Ok(zlib_include) = env::var("ZLIB_INCLUDE_DIR") {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", zlib_include));
    }

    if let Ok(expat_include) = env::var("EXPAT_INCLUDE_DIR") {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", expat_include));
    }

    // Add common include paths
    let bundled_include = manifest_dir.join("bundled").join("include");
    if bundled_include.exists() {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", bundled_include.display()));
    }

    // Add system include paths for dependencies like zlib and expat
    if cfg!(target_os = "linux") {
        bindgen_builder = bindgen_builder
            .clang_arg("-I/usr/include")
            .clang_arg("-I/usr/local/include");
    } else if cfg!(target_os = "macos") {
        bindgen_builder = bindgen_builder
            .clang_arg("-I/usr/local/include")
            .clang_arg("-I/opt/homebrew/include");
    }

    let bindings = bindgen_builder
        // Allowlist functions from MdfExport.cpp
        .allowlist_function("MdfReader.*")
        .allowlist_function("MdfWriter.*")
        .allowlist_function("MdfFile.*")
        .allowlist_function("MdfHeader.*")
        .allowlist_function("MdfDataGroup.*")
        .allowlist_function("MdfChannel.*")
        .allowlist_function("MdfChannelGroup.*")
        .allowlist_function("MdfChannelObserver.*")
        .allowlist_function("MdfChannelArray.*")
        .allowlist_function("MdfChannelConversion.*")
        .allowlist_function("MdfSourceInformation.*")
        .allowlist_function("MdfAttachment.*")
        .allowlist_function("MdfFileHistory.*")
        .allowlist_function("MdfEvent.*")
        .allowlist_function("MdfETag.*")
        .allowlist_function("MdfMetaData.*")
        .allowlist_function("CanMessage.*")
        // Allowlist types
        .allowlist_type("MdfWriterType")
        .allowlist_type("ChannelType")
        .allowlist_type("ChannelDataType")
        .allowlist_type("ChannelSyncType")
        .allowlist_type("ConversionType")
        .allowlist_type("ArrayType")
        .allowlist_type("ArrayStorage")
        .allowlist_type("SourceType")
        .allowlist_type("BusType")
        .allowlist_type("EventType")
        .allowlist_type("SyncType")
        .allowlist_type("RangeType")
        .allowlist_type("EventCause")
        .allowlist_type("ETagDataType")
        .allowlist_type("CanErrorType")
        .allowlist_type("MdfStorageType")
        // Generate useful derives
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .derive_partialeq(true)
        .derive_partialord(true)
        // Handle C++ specifics
        .prepend_enum_name(false)
        .layout_tests(false) // Disable layout tests for C++ structs
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn print_zlib_install_instructions() {
    if cfg!(target_os = "linux") {
        eprintln!("  Ubuntu/Debian: sudo apt install zlib1g-dev");
        eprintln!("  CentOS/RHEL/Fedora: sudo dnf install zlib-devel");
    } else if cfg!(target_os = "macos") {
        eprintln!("  macOS: brew install zlib");
    } else if cfg!(target_os = "windows") {
        eprintln!("  Windows: vcpkg install zlib:x64-windows");
    }
    eprintln!("\nAlternatively, set environment variables:");
    eprintln!("  ZLIB_LIBRARY=/path/to/libz.so (or .a/.lib)");
    eprintln!("  ZLIB_INCLUDE_DIR=/path/to/zlib/headers");
}

fn print_expat_install_instructions() {
    if cfg!(target_os = "linux") {
        eprintln!("  Ubuntu/Debian: sudo apt install libexpat1-dev");
        eprintln!("  CentOS/RHEL/Fedora: sudo dnf install expat-devel");
    } else if cfg!(target_os = "macos") {
        eprintln!("  macOS: brew install expat");
    } else if cfg!(target_os = "windows") {
        eprintln!("  Windows: vcpkg install expat:x64-windows");
    }
    eprintln!("\nAlternatively, set environment variables:");
    eprintln!("  EXPAT_LIBRARY=/path/to/libexpat.so (or .a/.lib)");
    eprintln!("  EXPAT_INCLUDE_DIR=/path/to/expat/headers");
}
