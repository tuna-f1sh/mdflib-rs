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
        setup_system_linking(&manifest_dir);
    } else {
        panic!("Either 'bundled' or 'system' feature must be enabled");
    }

    // Generate bindings
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

    // Configure with CMake
    let mut cmake_config = Command::new("cmake");
    cmake_config
        .current_dir(&build_dir)
        .arg(&bundled_dir)
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", install_dir.display()))
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DBUILD_SHARED_LIBS=OFF")
        .arg("-DMDF_BUILD_SHARED_LIB=OFF")
        .arg("-DMDF_BUILD_SHARED_LIB_NET=OFF")
        .arg("-DMDF_BUILD_TEST=OFF")
        .arg("-DMDF_BUILD_DOC=OFF")
        .arg("-DMDF_BUILD_TOOLS=OFF")
        .arg("-DCMAKE_CXX_STANDARD=17");

    // Platform-specific CMake settings
    if cfg!(target_os = "windows") && cfg!(target_env = "msvc") {
        cmake_config.arg("-G").arg("Visual Studio 16 2019");
        if cfg!(target_arch = "x86_64") {
            cmake_config.arg("-A").arg("x64");
        } else if cfg!(target_arch = "x86") {
            cmake_config.arg("-A").arg("Win32");
        }
    } else {
        cmake_config.arg("-G").arg("Unix Makefiles");
    }

    // Help CMake find dependencies
    add_dependency_hints(&mut cmake_config);

    // Run CMake configure
    let cmake_output = cmake_config
        .output()
        .expect("Failed to run CMake configure");
    if !cmake_output.status.success() {
        eprintln!("CMake configure failed:");
        eprintln!("stdout: {}", String::from_utf8_lossy(&cmake_output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&cmake_output.stderr));
        panic_with_dependency_errors(&String::from_utf8_lossy(&cmake_output.stderr));
    }

    // Run CMake build and install
    let mut cmake_build = Command::new("cmake");
    cmake_build
        .current_dir(&build_dir)
        .arg("--build")
        .arg(".")
        .arg("--config")
        .arg("Release")
        .arg("--target")
        .arg("install");

    if let Ok(jobs) = env::var("NUM_JOBS") {
        cmake_build.arg("--parallel").arg(jobs);
    } else if let Ok(jobs) = std::thread::available_parallelism() {
        cmake_build.arg("--parallel").arg(jobs.get().to_string());
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
    cc::Build::new()
        .cpp(true)
        .file("src/mdf_c_wrapper.cpp")
        .include(install_dir.join("include"))
        .include(bundled_dir.join("include"))
        .flag("-Wno-overloaded-virtual")
        .flag("-std=c++17")
        .compile("mdf_c_wrapper");

    // Set up linking
    setup_bundled_linking(&install_dir);
}

fn setup_dependencies() {
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    setup_dependency("zlib", "z");
    setup_dependency("expat", "expat");
}

fn setup_dependency(name: &str, fallback_name: &str) {
    let upper_name = name.to_uppercase();
    println!("cargo:rerun-if-env-changed={upper_name}_LIBRARY");
    println!("cargo:rerun-if-env-changed={upper_name}_INCLUDE_DIR");

    // Try pkg-config first
    if pkg_config::probe_library(name).is_ok() {
        println!("Found {name} via pkg-config");
        return;
    }

    // Then try environment variables
    if let Ok(lib_path_str) = env::var(format!("{upper_name}_LIBRARY")) {
        println!("Found {name} via {upper_name}_LIBRARY environment variable");
        let lib_path = PathBuf::from(&lib_path_str);
        if let Some(lib_dir) = lib_path.parent() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
        }
        if let Some(lib_name) = lib_path.file_stem() {
            let lib_name_str = lib_name.to_string_lossy();
            let clean_name = lib_name_str.strip_prefix("lib").unwrap_or(&lib_name_str);
            println!("cargo:rustc-link-lib={clean_name}");
        }
        return;
    }

    // Finally, fallback to default system linking
    println!(
        "cargo:warning={name} not found via pkg-config or environment variables, using system defaults"
    );
    println!("cargo:rustc-link-lib={fallback_name}");
}

fn add_dependency_hints(cmake_config: &mut Command) {
    add_single_dependency_hint(cmake_config, "ZLIB");
    add_single_dependency_hint(cmake_config, "EXPAT");

    if env::var("ZLIB_LIBRARY").is_err() && env::var("EXPAT_LIBRARY").is_err() {
        add_platform_dependency_hints(cmake_config);
    }
}

fn add_single_dependency_hint(cmake_config: &mut Command, name: &str) {
    if let Ok(lib_path_str) = env::var(format!("{name}_LIBRARY")) {
        let lib_path = PathBuf::from(&lib_path_str);
        if let Some(lib_dir) = lib_path.parent() {
            if let Some(root_dir) = lib_dir.parent() {
                cmake_config.arg(format!("-D{}_ROOT={}", name, root_dir.display()));
            }
        }
        cmake_config.arg(format!("-D{name}_LIBRARY={lib_path_str}"));
    }
    if let Ok(include_path) = env::var(format!("{name}_INCLUDE_DIR")) {
        cmake_config.arg(format!("-D{name}_INCLUDE_DIR={include_path}"));
    }
}

fn add_platform_dependency_hints(cmake_config: &mut Command) {
    if cfg!(target_os = "macos") {
        if Path::new("/opt/homebrew/opt/zlib").exists() {
            cmake_config.arg("-DZLIB_ROOT=/opt/homebrew/opt");
            cmake_config.arg("-DZLIB_LIBRARY=/opt/homebrew/opt/zlib/lib/libz.a");
        }
        if Path::new("/opt/homebrew/opt/expat").exists() {
            cmake_config.arg("-DEXPAT_ROOT=/opt/homebrew/opt");
            cmake_config.arg("-DEXPAT_LIBRARY=/opt/homebrew/opt/expat/lib/libexpat.a");
        }
        if Path::new("/usr/include/zlib.h").exists() {
            cmake_config.arg("-DZLIB_ROOT=/usr");
        }
        if Path::new("/usr/include/expat.h").exists() {
            cmake_config.arg("-DEXPAT_ROOT=/usr");
        }
    }
}

fn setup_bundled_linking(install_dir: &Path) {
    let lib_dir = install_dir.join("lib");
    if lib_dir.exists() {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
    }
    if install_dir.join("lib64").exists() {
        println!(
            "cargo:rustc-link-search=native={}",
            install_dir.join("lib64").display()
        );
    }

    // Link the static libraries in the correct order
    println!("cargo:rustc-link-lib=static=mdf_c_wrapper");
    println!("cargo:rustc-link-lib=static=mdf");

    // Link dependencies after the main libraries
    setup_dependencies();

    // Link platform-specific system libraries
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
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=dylib=System");
        println!("cargo:rustc-link-lib=framework=Foundation");
    }
}

fn setup_system_linking(_manifest_dir: &Path) {
    let mut cc_build = cc::Build::new();
    cc_build
        .cpp(true)
        .file("src/mdf_c_wrapper.cpp")
        .flag("-Wno-overloaded-virtual")
        .flag("-std=c++17");

    // Try to find system-installed mdflib using pkg-config
    if let Ok(library) = pkg_config::Config::new()
        .atleast_version("2.3")
        .probe("mdflib")
    {
        for path in library.include_paths {
            cc_build.include(path);
        }
    } else {
        println!("cargo:warning=pkg-config failed for mdflib, trying manual discovery");
        println!("cargo:rustc-link-lib=mdf");

        // Link dependencies after the main library
        setup_dependencies();

        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=dylib=m");
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=dl");
            cc_build.include("/usr/local/include");
            cc_build.include("/usr/include");
        } else if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-lib=dylib=c++");
            println!("cargo:rustc-link-lib=dylib=System");
            println!("cargo:rustc-link-lib=framework=Foundation");
            cc_build.include("/usr/local/include");
            cc_build.include("/opt/homebrew/include");
        } else if cfg!(target_os = "windows") {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=kernel32");
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=advapi2");
            println!("cargo:rustc-link-lib=dylib=shell32");
            println!("cargo:rustc-link-lib=dylib=ole32");
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
        .clang_arg("-std=c++17")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .blocklist_type("std::.*")
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .derive_partialeq(true)
        .derive_partialord(true);

    // Add include paths for bindgen
    let bundled_include = manifest_dir.join("bundled").join("include");
    if bundled_include.exists() {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", bundled_include.display()));
    }
    if let Ok(install_dir) = env::var("OUT_DIR") {
        let include_path = PathBuf::from(install_dir).join("install/include");
        if include_path.exists() {
            bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", include_path.display()));
        }
    }
    if let Ok(zlib_include) = env::var("ZLIB_INCLUDE_DIR") {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{zlib_include}"));
    }
    if let Ok(expat_include) = env::var("EXPAT_INCLUDE_DIR") {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{expat_include}"));
    }
    if cfg!(target_os = "macos") {
        if Path::new("/opt/homebrew/include").exists() {
            bindgen_builder = bindgen_builder.clang_arg("-I/opt/homebrew/include");
        }
        if let Ok(output) = Command::new("xcrun").args(["--show-sdk-path"]).output() {
            if output.status.success() {
                let sdk_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                bindgen_builder = bindgen_builder.clang_arg(format!("-I{sdk_path}/usr/include"));
            }
        }
    }

    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("Successfully generated bindings");
}

fn panic_with_dependency_errors(stderr: &str) {
    if stderr.contains("Could NOT find ZLIB") {
        eprintln!("\nzlib not found. Please install zlib development libraries:");
        print_install_instructions("zlib1g-dev", "zlib-devel");
    }
    if stderr.contains("Could NOT find EXPAT") {
        eprintln!("\nexpat not found. Please install expat development libraries:");
        print_install_instructions("libexpat1-dev", "expat-devel");
    }
    panic!("CMake configuration failed");
}

fn print_install_instructions(debian_pkg: &str, rhel_pkg: &str) {
    if cfg!(target_os = "linux") {
        eprintln!("  Ubuntu/Debian: sudo apt install {debian_pkg}");
        eprintln!("  CentOS/RHEL/Fedora: sudo dnf install {rhel_pkg}");
    }
    let name = rhel_pkg.split('-').next().unwrap_or("").to_uppercase();
    eprintln!("\nAlternatively, set environment variables:");
    eprintln!(
        "  {}_LIBRARY=/path/to/lib{}.so (or .a/.lib)",
        name,
        rhel_pkg.replace("-devel", "")
    );
    eprintln!(
        "  {}_INCLUDE_DIR=/path/to/{}/headers",
        name,
        rhel_pkg.replace("-devel", "")
    );
}
