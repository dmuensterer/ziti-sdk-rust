use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // If ZITI_BUILD_DIR is set, skip cmake and use pre-built libs
    let build_dir = if let Ok(dir) = env::var("ZITI_BUILD_DIR") {
        PathBuf::from(dir)
    } else {
        build_with_cmake()
    };

    // Link libziti + tlsuv from build dir
    let ziti_lib_dir = build_dir.join("library");
    let tlsuv_lib_dir = build_dir.join("_deps/tlsuv-build");
    println!("cargo:rustc-link-search=native={}", ziti_lib_dir.display());
    println!("cargo:rustc-link-search=native={}", tlsuv_lib_dir.display());
    println!("cargo:rustc-link-lib=static=ziti");
    println!("cargo:rustc-link-lib=static=tlsuv");

    // vcpkg-installed static libs
    let vcpkg_installed = build_dir.join("vcpkg_installed");
    if let Some(vlib) = find_vcpkg_lib(&vcpkg_installed) {
        println!("cargo:rustc-link-search=native={}", vlib.display());
        for lib in &["uv", "sodium", "json-c", "protobuf-c", "llhttp", "ssl", "crypto", "z", "stc"] {
            println!("cargo:rustc-link-lib=static={lib}");
        }
    }

    // OS-level libraries
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        println!("cargo:rustc-link-lib=dylib=resolv");
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    } else {
        println!("cargo:rustc-link-lib=dylib=pthread");
        println!("cargo:rustc-link-lib=dylib=dl");
        println!("cargo:rustc-link-lib=dylib=m");
    }

    println!("cargo:rerun-if-changed=../../vendor/ziti-sdk-c/library");
}

fn build_with_cmake() -> PathBuf {
    let sdk_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../../vendor/ziti-sdk-c")
        .canonicalize()
        .expect("vendor/ziti-sdk-c not found - run `git submodule update --init`");

    let vcpkg_root = env::var("VCPKG_ROOT")
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("HOME not set");
            format!("{home}/vcpkg")
        });
    let toolchain = format!("{vcpkg_root}/scripts/buildsystems/vcpkg.cmake");

    if !PathBuf::from(&toolchain).exists() {
        panic!("vcpkg toolchain not found at {toolchain}. Set VCPKG_ROOT or install vcpkg.");
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("ziti-build");
    std::fs::create_dir_all(&build_dir).unwrap();

    let status = Command::new("cmake")
        .args([
            "-S", sdk_dir.to_str().unwrap(),
            "-B", build_dir.to_str().unwrap(),
            &format!("-DCMAKE_TOOLCHAIN_FILE={toolchain}"),
            "-DBUILD_SHARED_LIBS=OFF",
            "-DCMAKE_BUILD_TYPE=Release",
        ])
        .status()
        .expect("cmake configure failed");
    assert!(status.success(), "cmake configure failed");

    let status = Command::new("cmake")
        .args([
            "--build", build_dir.to_str().unwrap(),
            "--config", "Release",
            "--target", "ziti",
            "-j", &num_cpus(),
        ])
        .status()
        .expect("cmake build failed");
    assert!(status.success(), "cmake build failed");

    build_dir
}

fn num_cpus() -> String {
    std::thread::available_parallelism()
        .map(|n| n.get().to_string())
        .unwrap_or_else(|_| "4".to_string())
}

fn find_vcpkg_lib(vcpkg_installed: &Path) -> Option<PathBuf> {
    for triplet in &["arm64-osx", "x64-osx", "x64-linux", "arm64-linux"] {
        let p = vcpkg_installed.join(triplet).join("lib");
        if p.exists() {
            return Some(p);
        }
    }
    None
}
