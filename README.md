# hwinfo-rs

Safe Rust bindings for the cross-platform
[`lfreist/hwinfo`](https://github.com/lfreist/hwinfo) C++ library.

The default API collects CPU, GPU, memory, operating-system, battery, network,
motherboard, and physical-disk information on Linux, macOS, and Windows. It
uses a narrow C ABI internally, so no C++ STL types, exceptions, or native
allocations are exposed through the public Rust API.

## Clone

The upstream C++ project is pinned as a Git submodule:

```sh
git clone --recurse-submodules <your-hwinfo-rs-repository-url>
```

For an existing clone:

```sh
git submodule update --init --recursive
```

## Use

```rust
let hardware = hwinfo_rs::collect()?;

println!("board serial: {:?}", hardware.mainboard.and_then(|board| board.serial_number));
for disk in hardware.disks {
    println!("disk serial: {:?}", disk.serial_number);
}
for error in hardware.errors {
    eprintln!("component unavailable: {error}");
}
# Ok::<(), hwinfo_rs::CollectionError>(())
```

Each component also has an independent collector, such as
`hwinfo_rs::cpu::collect()` or `hwinfo_rs::mainboard::collect()`. Aggregate
collection is deliberately partial: a failed platform API is recorded in
`HardwareInfo::errors` without discarding the other component snapshots.

## Features

All base components and synchronous monitoring are enabled by default through
the `full` feature. Consumers can trim native code, for example:

```toml
hwinfo-rs = { path = "../hwinfo-rs", default-features = false, features = ["cpu", "mainboard", "disk"] }
```

Available features are `cpu`, `gpu`, `memory`, `os`, `battery`, `network`,
`mainboard`, `disk`, and `monitoring`. The optional `opencl` feature enriches
base GPU records with driver, memory, clock, and core data where a matching
OpenCL device exists; it requires target OpenCL C/C++ headers and a loader.

Monitoring is synchronous and leaves scheduling to the caller:

```rust
let cpu = hwinfo_rs::monitoring::cpu(std::time::Duration::from_millis(200))?;
let memory = hwinfo_rs::monitoring::memory()?;
let root = hwinfo_rs::monitoring::disk("/")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

CPU monitoring blocks for the requested sampling duration.

## Native build requirements

- A C++17 compiler for the Cargo target
- Linux: libstdc++
- macOS: libc++, CoreFoundation, and IOKit from the target SDK
- Windows GNU: MinGW-w64 GCC 13 or newer with its C++ compiler and WMI import
  libraries. The C++ runtime is linked statically.

Cargo's `cc` build helper honors target-qualified compiler variables. The
Nebula Docker image should provide at least:

```text
CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++-win32
CXX_x86_64_apple_darwin=o64-clang++
CXX_aarch64_apple_darwin=oa64-clang++
```

This crate gathers data when the resulting application runs. Building it in a
container does not fingerprint the build container.

The repository includes a cross-link check using the same target families and
osxcross image as Nebula:

```sh
docker build -f Dockerfile.cross .
```

## Platform behavior

- Linux and Windows expose the broadest inventory surface.
- Upstream currently returns no GPU or network records on macOS.
- Some Windows battery fields and Apple CPU monitoring values are limited by
  the upstream implementation.
- Missing devices are represented by empty lists. Actual collection failures
  appear in the typed error list.

## Hardware fingerprints and licensing

Hardware fields are useful inputs to a licensing fingerprint, but no single
field is a permanent or secret identifier. Firmware updates, VM cloning,
hardware replacement, permissions, and spoofing can change or hide values.
Normalize several stable inputs, hash them with an application-specific salt,
allow a recovery/rebinding path, and treat the result as personal data where
applicable. This crate intentionally does not invent a machine UUID or OS
installation identifier beyond the upstream library's inventory.

## Licensing

`hwinfo-rs` is MIT licensed. Its vendored `lfreist/hwinfo` dependency is also
MIT licensed; see `THIRD_PARTY_LICENSES/hwinfo.txt`.
