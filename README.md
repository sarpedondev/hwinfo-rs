# hwinfo-rs

Safe Rust bindings for the cross-platform
[`lfreist/hwinfo`](https://github.com/lfreist/hwinfo) C++ library.

The initial API collects motherboard and physical-disk information on Linux,
macOS, and Windows. It uses a narrow C ABI internally, so no C++ STL types or
native allocations are exposed through the public Rust API.

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

println!("board serial: {:?}", hardware.mainboard.serial_number);
for disk in hardware.disks {
    println!("disk serial: {:?}", disk.serial_number);
}
# Ok::<(), hwinfo_rs::CollectionError>(())
```

## Native build requirements

- A C++17 compiler for the Cargo target
- Linux: libstdc++
- macOS: libc++, CoreFoundation, and IOKit from the target SDK
- Windows GNU: MinGW-w64 with its C++ compiler and WMI import libraries

Cargo's `cc` build helper honors target-qualified compiler variables. The
Nebula Docker image should provide at least:

```text
CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
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

## Current scope

- Motherboard vendor, name, version, and serial number
- Physical disk identity, size, interface, and mount points

CPU, memory, GPU, OS, battery, network, and monitoring bindings can be added
without changing the public ownership model.

## Licensing

`hwinfo-rs` is MIT licensed. Its vendored `lfreist/hwinfo` dependency is also
MIT licensed; see `THIRD_PARTY_LICENSES/hwinfo.txt`.
