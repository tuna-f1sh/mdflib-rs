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

fn apply_mingw_patches(bundled_dir: &Path) {
    use std::fs;
    
    // Patch blockproperty.h to add missing #include <cstdint>
    // This issue was fixed in upstream commit 3c49205, but we keep this patch
    // for backwards compatibility with older submodule versions
    let blockproperty_h = bundled_dir.join("mdflib/src/blockproperty.h");
    if blockproperty_h.exists() {
        let content = fs::read_to_string(&blockproperty_h)
            .expect("Failed to read blockproperty.h");
        
        // Only patch if not already fixed
        if !content.contains("#include <cstdint>") && content.contains("int64_t") {
            let lines: Vec<&str> = content.lines().collect();
            let mut new_content = String::new();
            let mut patched = false;
            
            for (i, line) in lines.iter().enumerate() {
                new_content.push_str(line);
                new_content.push('\n');
                
                // Add #include <cstdint> after #include <string>
                if !patched && line.contains("#include <string>") && i + 1 < lines.len() {
                    new_content.push_str("#include <cstdint>\n");
                    patched = true;
                }
            }
            
            if patched {
                fs::write(&blockproperty_h, new_content)
                    .expect("Failed to write patched blockproperty.h");
                println!("cargo:warning=Applied MinGW patch to blockproperty.h (added cstdint include)");
            }
        }
    }
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

    // Apply patches for MinGW/GCC compatibility
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("gnu") || target.contains("mingw") {
        apply_mingw_patches(&bundled_dir);
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
    // The TARGET env var is always set by Cargo and tells us what we're compiling for
    let target = env::var("TARGET").unwrap_or_default();
    
    if target.contains("windows") {
        if target.contains("msvc") {
            // MSVC target uses Visual Studio generator
            // Try different Visual Studio versions
            let vs_generators = vec![
                ("Visual Studio 17 2022", "VS 2022"),
                ("Visual Studio 16 2019", "VS 2019"),
                ("Visual Studio 15 2017", "VS 2017"),
            ];
            
            let mut found_vs = false;
            for (generator, name) in &vs_generators {
                // Test if this generator is available
                let test_output = Command::new("cmake")
                    .arg("-G")
                    .arg(generator)
                    .arg("--help")
                    .output();
                    
                if test_output.map_or(false, |o| o.status.success()) {
                    println!("cargo:warning=Using CMake generator: {} ({})", generator, name);
                    cmake_config.arg("-G").arg(generator);
                    found_vs = true;
                    break;
                }
            }
            
            if !found_vs {
                eprintln!("\nERROR: No Visual Studio installation found for MSVC target.");
                eprintln!("Please install Visual Studio 2017 or later with C++ support.");
                eprintln!("\nAlternatively, use the GNU toolchain:");
                eprintln!("  rustup target add x86_64-pc-windows-gnu");
                eprintln!("  cargo build --target x86_64-pc-windows-gnu");
                panic!("Visual Studio not found for MSVC target");
            }
            
            if target.contains("x86_64") {
                cmake_config.arg("-A").arg("x64");
            } else if target.contains("i686") || target.contains("i586") {
                cmake_config.arg("-A").arg("Win32");
            }
        } else if target.contains("gnu") {
            // GNU target (MinGW) uses MinGW Makefiles
            println!("cargo:warning=Using MinGW Makefiles generator for GNU target");
            cmake_config.arg("-G").arg("MinGW Makefiles");
        } else {
            // Fallback for other Windows toolchains
            println!("cargo:warning=Unknown Windows toolchain, trying MinGW Makefiles");
            cmake_config.arg("-G").arg("MinGW Makefiles");
        }
    } else {
        // Unix systems use Unix Makefiles
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
    let mut cc_build = cc::Build::new();
    cc_build
        .cpp(true)
        .file("src/mdf_c_wrapper.cpp")
        .include(install_dir.join("include"))
        .include(bundled_dir.join("include"));
    
    // Add compiler-specific flags
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("msvc") {
        cc_build.flag("/std:c++17");
    } else {
        cc_build.flag("-Wno-overloaded-virtual");
        cc_build.flag("-std=c++17");
    }
    
    cc_build.compile("mdf_c_wrapper");

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

    let target = env::var("TARGET").unwrap_or_default();
    let is_musl = target.contains("musl");
    let is_windows = target.contains("windows");
    
    // Determine link type based on target
    let link_type = if is_musl || (is_windows && target.contains("static")) {
        "static"
    } else if is_windows {
        // On Windows, vcpkg provides static libraries with -static triplets
        if env::var("VCPKG_ROOT").is_ok() {
            "static"
        } else {
            "dylib"
        }
    } else {
        "dylib"
    };

    // Try pkg-config first (works on Linux/macOS)
    if !is_windows {
        let mut pkg_config = pkg_config::Config::new();
        if is_musl {
            pkg_config.statik(true);
        }
        if pkg_config.probe(name).is_ok() {
            println!("cargo:warning=Found {name} via pkg-config");
            return;
        }
    }

    // Try environment variables
    if let Ok(lib_path_str) = env::var(format!("{upper_name}_LIBRARY")) {
        println!("cargo:warning=Found {name} via {upper_name}_LIBRARY environment variable");
        let lib_path = PathBuf::from(&lib_path_str);
        if let Some(lib_dir) = lib_path.parent() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
        }
        if let Some(lib_name) = lib_path.file_stem() {
            let lib_name_str = lib_name.to_string_lossy();
            let clean_name = lib_name_str.strip_prefix("lib").unwrap_or(&lib_name_str);
            println!("cargo:rustc-link-lib={link_type}={clean_name}");
        }
        return;
    }

    // Try vcpkg on Windows
    if is_windows {
        if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
            let vcpkg_path = PathBuf::from(&vcpkg_root);
            let triplet = if target.contains("msvc") {
                if target.contains("x86_64") {
                    "x64-windows-static"
                } else {
                    "x86-windows-static"
                }
            } else {
                // MinGW
                if target.contains("x86_64") {
                    "x64-mingw-static"
                } else {
                    "x86-mingw-static"
                }
            };
            
            let vcpkg_lib_dir = vcpkg_path.join("installed").join(triplet).join("lib");
            if vcpkg_lib_dir.exists() {
                println!("cargo:rustc-link-search=native={}", vcpkg_lib_dir.display());
                
                // Determine the actual library name in vcpkg
                let lib_name = if name == "zlib" {
                    if target.contains("msvc") { "zlib" } else { "z" }
                } else {
                    fallback_name
                };
                
                println!("cargo:rustc-link-lib=static={}", lib_name);
                println!("cargo:warning=Found {name} via vcpkg at {}", vcpkg_lib_dir.display());
                return;
            }
        }
    }

    // For musl, try to find static libraries in standard locations
    if is_musl {
        let static_lib_name = format!("lib{}.a", fallback_name);
        let search_paths = vec![
            PathBuf::from("/usr/lib/x86_64-linux-musl"),
            PathBuf::from("/usr/lib"),
            PathBuf::from("/usr/local/lib"),
        ];

        for search_path in search_paths {
            let lib_path = search_path.join(&static_lib_name);
            if lib_path.exists() {
                println!("cargo:rustc-link-search=native={}", search_path.display());
                println!("cargo:rustc-link-lib=static={fallback_name}");
                println!("cargo:warning=Found {name} static library at {}", lib_path.display());
                return;
            }
        }
    }

    // Finally, fallback to default system linking
    println!(
        "cargo:warning={name} not found via pkg-config or environment variables, using system defaults"
    );
    println!("cargo:rustc-link-lib={link_type}={fallback_name}");
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
    let target = env::var("TARGET").unwrap_or_default();
    
    if target.contains("apple") || target.contains("darwin") {
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
    } else if target.contains("linux") {
        // Arch Linux typically has zlib and expat in /usr/include and /usr/lib, so we can hint CMake to look there
        if Path::new("/usr/include/zlib.h").exists() {
            cmake_config.arg("-DZLIB_ROOT=/usr");
        }
        if Path::new("/usr/include/expat.h").exists() {
            cmake_config.arg("-DEXPAT_ROOT=/usr");
        }
        if Path::new("/usr/lib").exists() {
            cmake_config.arg("-DEXPAT_LIBRARY=/usr/lib/libexpat.so");
            cmake_config.arg("-DZLIB_LIBRARY=/usr/lib/libz.so");
        }
    } else if target.contains("windows") {
        // For Windows with vcpkg, try to find the dependencies
        if let Ok(vcpkg_root) = env::var("VCPKG_ROOT") {
            let vcpkg_path = PathBuf::from(&vcpkg_root);
            let triplet = if target.contains("msvc") {
                if target.contains("x86_64") {
                    "x64-windows-static"
                } else {
                    "x86-windows-static"
                }
            } else {
                // MinGW
                if target.contains("x86_64") {
                    "x64-mingw-static"
                } else {
                    "x86-mingw-static"
                }
            };
            
            let vcpkg_installed = vcpkg_path.join("installed").join(triplet);
            if vcpkg_installed.exists() {
                cmake_config.arg(format!("-DCMAKE_PREFIX_PATH={}", vcpkg_installed.display()));
            }
            
            // Always set the toolchain file for vcpkg
            let toolchain_file = vcpkg_path.join("scripts").join("buildsystems").join("vcpkg.cmake");
            if toolchain_file.exists() {
                cmake_config.arg(format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain_file.display()));
                cmake_config.arg(format!("-DVCPKG_TARGET_TRIPLET={}", triplet));
            }
        }
    }
}

fn setup_bundled_linking(install_dir: &Path) {
    let target = env::var("TARGET").unwrap_or_default();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("build");
    
    // Collect all possible library paths
    let mut lib_search_paths = Vec::new();
    
    // Standard lib directory
    let lib_dir = install_dir.join("lib");
    if lib_dir.exists() {
        lib_search_paths.push(lib_dir.clone());
    }
    
    // lib64 variant
    if install_dir.join("lib64").exists() {
        lib_search_paths.push(install_dir.join("lib64"));
    }
    
    // For MSVC, check multiple possible locations
    if target.contains("msvc") {
        // Install directory subdirectories
        let lib_release = lib_dir.join("Release");
        let lib_debug = lib_dir.join("Debug");
        if lib_release.exists() {
            lib_search_paths.push(lib_release);
        }
        if lib_debug.exists() {
            lib_search_paths.push(lib_debug);
        }
        
        // Build directory (where MSVC actually puts the files)
        let build_mdflib_release = build_dir.join("mdflib").join("Release");
        let build_mdflib_debug = build_dir.join("mdflib").join("Debug");
        if build_mdflib_release.exists() {
            lib_search_paths.push(build_mdflib_release);
        }
        if build_mdflib_debug.exists() {
            lib_search_paths.push(build_mdflib_debug);
        }
        
        // Also try build/lib/Release for older CMake versions
        let build_lib_release = build_dir.join("lib").join("Release");
        let build_lib_debug = build_dir.join("lib").join("Debug");
        if build_lib_release.exists() {
            lib_search_paths.push(build_lib_release);
        }
        if build_lib_debug.exists() {
            lib_search_paths.push(build_lib_debug);
        }
    }
    
    // Add all search paths and emit warnings for debugging
    for path in &lib_search_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
        
        // List what's actually in this directory for debugging
        if let Ok(entries) = std::fs::read_dir(path) {
            let files: Vec<String> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "lib" || ext == "a"))
                .map(|e| e.file_name().to_string_lossy().to_string())
                .collect();
            if !files.is_empty() {
                println!("cargo:warning=Libraries in {}: {}", path.display(), files.join(", "));
            }
        }
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
        .file("src/mdf_c_wrapper.cpp");
    
    // Add compiler-specific flags
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("msvc") {
        cc_build.flag("/std:c++17");
    } else {
        cc_build.flag("-Wno-overloaded-virtual");
        cc_build.flag("-std=c++17");
    }

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
