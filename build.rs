use std::{env, fs, path::PathBuf};

fn enabled(name: &str) -> bool {
    env::var_os(format!("CARGO_FEATURE_{}", name.to_ascii_uppercase())).is_some()
}

fn add_component(build: &mut cc::Build, upstream: &std::path::Path, platform: &str, name: &str) {
    build
        .file(upstream.join(format!("src/{name}.cpp")))
        .file(upstream.join(format!("src/{platform}/{name}.cpp")));
}

fn add_static_cpp_stdlib_search(build: &cc::Build, library: &str) {
    let output = build
        .get_compiler()
        .to_command()
        .arg(format!("-print-file-name=lib{library}.a"))
        .output()
        .expect("failed to locate the static C++ runtime");
    if !output.status.success() {
        panic!("C++ compiler failed to locate the static C++ runtime");
    }

    let archive = PathBuf::from(
        String::from_utf8(output.stdout)
            .expect("C++ runtime path is not UTF-8")
            .trim(),
    );
    let directory = archive
        .parent()
        .filter(|_| archive.is_file())
        .expect("C++ compiler did not return a static C++ runtime archive");
    println!("cargo:rustc-link-search=native={}", directory.display());
}

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let upstream = manifest_dir.join("vendor/hwinfo");
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("missing CARGO_CFG_TARGET_OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_abi = env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("missing OUT_DIR"));

    if !upstream.join("include/hwinfo/hwinfo.h").is_file() {
        panic!("the hwinfo submodule is missing; run `git submodule update --init --recursive`");
    }

    println!("cargo:rerun-if-changed=ffi/hwinfo_rs.cpp");
    println!("cargo:rerun-if-changed=include/hwinfo_rs.h");
    println!("cargo:rerun-if-changed=vendor/hwinfo/include");
    println!("cargo:rerun-if-changed=vendor/hwinfo/src");
    println!("cargo:rerun-if-changed=vendor/hwinfo/data/pci.ids");

    let mut native = cc::Build::new();
    native
        .cpp(true)
        .std("c++17")
        .define("HWINFO_STATIC", None)
        .include(manifest_dir.join("include"))
        .include(upstream.join("include"))
        .include(&out_dir)
        .file(manifest_dir.join("ffi/hwinfo_rs.cpp"));

    let platform = match target_os.as_str() {
        "linux" => "linux",
        "macos" => "apple",
        "windows" => "windows",
        unsupported => panic!("hwinfo-rs does not support target OS `{unsupported}`"),
    };

    for (feature, component) in [
        ("BATTERY", "battery"),
        ("CPU", "cpu"),
        ("DISK", "disk"),
        ("GPU", "gpu"),
        ("MAINBOARD", "mainboard"),
        ("OS", "os"),
        ("MEMORY", "ram"),
        ("NETWORK", "network"),
    ] {
        if enabled(feature) {
            native.define(&format!("HWINFO_RS_{feature}"), None);
            if target_os == "macos" && component == "battery" {
                native.file(upstream.join("src/battery.cpp"));
            } else {
                add_component(&mut native, &upstream, platform, component);
            }
        }
    }

    if enabled("MONITORING") {
        native.define("HWINFO_RS_MONITORING", None);
        for component in ["cpu", "ram", "disk"] {
            native.file(upstream.join(format!("src/{platform}/monitoring/{component}.cpp")));
        }
    }

    if enabled("GPU") {
        native.file(upstream.join("src/PCIMapper.cpp"));
        let pci_data = fs::read(upstream.join("data/pci.ids")).expect("failed to read pci.ids");
        let mut generated = String::from("const unsigned char pci_ids[] = {");
        for byte in pci_data {
            generated.push_str(&format!("0x{byte:02x},"));
        }
        generated.push_str("};\nconst unsigned int pci_ids_size = sizeof(pci_ids);\n");
        fs::write(out_dir.join("pci.ids.h"), generated).expect("failed to generate pci.ids.h");
    }

    if enabled("OPENCL") && matches!(target_os.as_str(), "linux" | "windows") {
        native
            .define("HWINFO_RS_OPENCL", None)
            .file(upstream.join("src/opencl/device.cpp"));
        println!("cargo:rustc-link-lib=OpenCL");
    }

    match target_os.as_str() {
        "linux" => {
            native.pic(true).cpp_link_stdlib("stdc++");
        }
        "macos" => {
            native.pic(true).cpp_link_stdlib("c++");
            native.define("getVendor", "getCpuVendor");
            if env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("x86_64") && enabled("CPU") {
                native.flag("-include").flag(
                    manifest_dir
                        .join("compat/apple/cpuid.h")
                        .to_str()
                        .expect("non-UTF-8 compatibility path"),
                );
            }
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=IOKit");
        }
        "windows" => {
            if target_env == "gnu" {
                let llvm = target_abi == "llvm";
                let library = if llvm { "c++" } else { "stdc++" };
                native.cpp_link_stdlib(library).cpp_link_stdlib_static(true);
                add_static_cpp_stdlib_search(&native, library);
                if llvm {
                    // LLVM-MinGW splits the static C++ runtime across these archives.
                    println!("cargo:rustc-link-lib=static=c++abi");
                    println!("cargo:rustc-link-lib=static=unwind");
                }
            } else {
                native.cpp_link_stdlib(None);
            }
            if enabled("MAINBOARD")
                || enabled("MEMORY")
                || enabled("OS")
                || enabled("BATTERY")
                || enabled("NETWORK")
            {
                native.file(upstream.join("src/windows/utils/wmi_wrapper.cpp"));
                println!("cargo:rustc-link-lib=ole32");
                println!("cargo:rustc-link-lib=oleaut32");
                println!("cargo:rustc-link-lib=wbemuuid");
            }
            if target_env == "gnu" {
                native.include(manifest_dir.join("compat/mingw"));
            }
            if enabled("CPU") || enabled("MONITORING") {
                println!("cargo:rustc-link-lib=powrprof");
                println!("cargo:rustc-link-lib=ntdll");
                println!("cargo:rustc-link-lib=advapi32");
            }
            if enabled("GPU") {
                println!("cargo:rustc-link-lib=dxgi");
                println!("cargo:rustc-link-lib=setupapi");
            }
        }
        _ => unreachable!(),
    }

    native.compile("hwinfo_rs_native");

    if target_os == "macos" && enabled("BATTERY") {
        let mut battery = cc::Build::new();
        battery
            .cpp(true)
            .std("c++17")
            .pic(true)
            .cpp_link_stdlib("c++")
            .define("HWINFO_STATIC", None)
            .define("getVendor", "getBatteryVendor")
            .include(upstream.join("include"))
            .file(upstream.join("src/apple/battery.cpp"))
            .compile("hwinfo_rs_apple_battery");
    }
}
