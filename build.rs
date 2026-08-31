use std::{env, path::PathBuf};

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let upstream = manifest_dir.join("vendor/hwinfo");
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("missing CARGO_CFG_TARGET_OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    if !upstream.join("include/hwinfo/hwinfo.h").is_file() {
        panic!("the hwinfo submodule is missing; run `git submodule update --init --recursive`");
    }

    println!("cargo:rerun-if-changed=ffi/hwinfo_rs.cpp");
    println!("cargo:rerun-if-changed=include/hwinfo_rs.h");
    println!("cargo:rerun-if-changed=vendor/hwinfo/include");
    println!("cargo:rerun-if-changed=vendor/hwinfo/src/mainboard.cpp");
    println!("cargo:rerun-if-changed=vendor/hwinfo/src/disk.cpp");

    let mut native = cc::Build::new();
    native
        .cpp(true)
        .std("c++17")
        .define("HWINFO_STATIC", None)
        .include(manifest_dir.join("include"))
        .include(upstream.join("include"))
        .file(manifest_dir.join("ffi/hwinfo_rs.cpp"))
        .file(upstream.join("src/mainboard.cpp"))
        .file(upstream.join("src/disk.cpp"));

    match target_os.as_str() {
        "linux" => {
            native
                .pic(true)
                .cpp_link_stdlib("stdc++")
                .file(upstream.join("src/linux/mainboard.cpp"))
                .file(upstream.join("src/linux/disk.cpp"));
        }
        "macos" => {
            native
                .pic(true)
                .cpp_link_stdlib("c++")
                .file(upstream.join("src/apple/mainboard.cpp"))
                .file(upstream.join("src/apple/disk.cpp"));
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=IOKit");
        }
        "windows" => {
            native
                .cpp_link_stdlib(if target_env == "gnu" {
                    Some("stdc++")
                } else {
                    None
                })
                .file(upstream.join("src/windows/mainboard.cpp"))
                .file(upstream.join("src/windows/disk.cpp"))
                .file(upstream.join("src/windows/utils/wmi_wrapper.cpp"));
            if target_env == "gnu" {
                native.include(manifest_dir.join("compat/mingw"));
            }
            println!("cargo:rustc-link-lib=ole32");
            println!("cargo:rustc-link-lib=oleaut32");
            println!("cargo:rustc-link-lib=wbemuuid");
        }
        unsupported => panic!("hwinfo-rs does not support target OS `{unsupported}`"),
    }

    native.compile("hwinfo_rs_native");
}
