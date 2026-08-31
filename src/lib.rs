//! Safe Rust bindings for selected functionality from `lfreist/hwinfo`.
//!
//! The first release exposes motherboard and physical-disk inventory. Native
//! objects never cross the public Rust API; values are copied into owned Rust
//! types before the underlying C++ snapshot is released.

use std::{
    error::Error as StdError,
    ffi::{CStr, c_char},
    fmt,
    ptr::NonNull,
};

mod sys {
    use std::ffi::c_char;

    #[repr(C)]
    pub struct Snapshot {
        _private: [u8; 0],
    }

    unsafe extern "C" {
        pub fn hwinfo_rs_last_error() -> *const c_char;

        pub fn hwinfo_rs_snapshot_collect() -> *mut Snapshot;
        pub fn hwinfo_rs_snapshot_free(snapshot: *mut Snapshot);

        pub fn hwinfo_rs_mainboard_vendor(snapshot: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_mainboard_name(snapshot: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_mainboard_version(snapshot: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_mainboard_serial(snapshot: *const Snapshot) -> *const c_char;

        pub fn hwinfo_rs_disk_count(snapshot: *const Snapshot) -> usize;
        pub fn hwinfo_rs_disk_id(snapshot: *const Snapshot, index: usize) -> u32;
        pub fn hwinfo_rs_disk_vendor(snapshot: *const Snapshot, index: usize) -> *const c_char;
        pub fn hwinfo_rs_disk_model(snapshot: *const Snapshot, index: usize) -> *const c_char;
        pub fn hwinfo_rs_disk_serial(snapshot: *const Snapshot, index: usize) -> *const c_char;
        pub fn hwinfo_rs_disk_size_bytes(snapshot: *const Snapshot, index: usize) -> u64;
        pub fn hwinfo_rs_disk_interface(snapshot: *const Snapshot, index: usize) -> u32;
        pub fn hwinfo_rs_disk_mount_point_count(snapshot: *const Snapshot, index: usize) -> usize;
        pub fn hwinfo_rs_disk_mount_point(
            snapshot: *const Snapshot,
            disk_index: usize,
            mount_index: usize,
        ) -> *const c_char;
    }
}

/// A complete hardware inventory snapshot supported by this crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HardwareInfo {
    pub mainboard: Mainboard,
    pub disks: Vec<Disk>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mainboard {
    pub vendor: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub serial_number: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Disk {
    pub id: u32,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_bytes: u64,
    pub interface: DiskInterface,
    pub mount_points: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DiskInterface {
    Nvme,
    Usb,
    Usb1,
    Usb2,
    Usb3FiveGbit,
    Usb3TenGbit,
    Usb3TwentyGbit,
    Usb4TwentyGbit,
    Usb4FortyGbit,
    Usb4EightyGbit,
    Sata,
    Scsi,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionError {
    message: String,
}

impl CollectionError {
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for CollectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "hardware collection failed: {}", self.message)
    }
}

impl StdError for CollectionError {}

struct NativeSnapshot(NonNull<sys::Snapshot>);

impl NativeSnapshot {
    fn collect() -> Result<Self, CollectionError> {
        // SAFETY: the native function has no arguments and returns either a
        // uniquely owned allocation or null. Ownership is released by Drop.
        let snapshot = unsafe { sys::hwinfo_rs_snapshot_collect() };
        NonNull::new(snapshot)
            .map(Self)
            .ok_or_else(|| CollectionError {
                // SAFETY: the native bridge owns a thread-local, nul-terminated
                // error string whose storage remains valid until the next call.
                message: unsafe { string_from_ptr(sys::hwinfo_rs_last_error()) }
                    .unwrap_or_else(|| "unknown native error".to_owned()),
            })
    }

    fn as_ptr(&self) -> *const sys::Snapshot {
        self.0.as_ptr()
    }
}

impl Drop for NativeSnapshot {
    fn drop(&mut self) {
        // SAFETY: this is the unique allocation returned by collect and Drop
        // runs exactly once for this owner.
        unsafe { sys::hwinfo_rs_snapshot_free(self.0.as_ptr()) }
    }
}

/// Collects motherboard and physical-disk information from the current host.
pub fn collect() -> Result<HardwareInfo, CollectionError> {
    let native = NativeSnapshot::collect()?;
    let snapshot = native.as_ptr();

    let mainboard = Mainboard {
        // SAFETY: all pointers refer to strings owned by `native`, which stays
        // alive until the complete Rust snapshot has been copied.
        vendor: unsafe { string_from_ptr(sys::hwinfo_rs_mainboard_vendor(snapshot)) },
        name: unsafe { string_from_ptr(sys::hwinfo_rs_mainboard_name(snapshot)) },
        version: unsafe { string_from_ptr(sys::hwinfo_rs_mainboard_version(snapshot)) },
        serial_number: unsafe { string_from_ptr(sys::hwinfo_rs_mainboard_serial(snapshot)) },
    };

    // SAFETY: `snapshot` is valid for the lifetime of `native`.
    let disk_count = unsafe { sys::hwinfo_rs_disk_count(snapshot) };
    let mut disks = Vec::with_capacity(disk_count);

    for index in 0..disk_count {
        // SAFETY: `index` is within the count returned by the same immutable
        // snapshot, and every string is copied before `native` is dropped.
        let disk = unsafe {
            let mount_count = sys::hwinfo_rs_disk_mount_point_count(snapshot, index);
            let mut mount_points = Vec::with_capacity(mount_count);
            for mount_index in 0..mount_count {
                if let Some(mount_point) = raw_string_from_ptr(sys::hwinfo_rs_disk_mount_point(
                    snapshot,
                    index,
                    mount_index,
                )) {
                    mount_points.push(mount_point);
                }
            }

            Disk {
                id: sys::hwinfo_rs_disk_id(snapshot, index),
                vendor: string_from_ptr(sys::hwinfo_rs_disk_vendor(snapshot, index)),
                model: string_from_ptr(sys::hwinfo_rs_disk_model(snapshot, index)),
                serial_number: string_from_ptr(sys::hwinfo_rs_disk_serial(snapshot, index)),
                size_bytes: sys::hwinfo_rs_disk_size_bytes(snapshot, index),
                interface: DiskInterface::from_native(sys::hwinfo_rs_disk_interface(
                    snapshot, index,
                )),
                mount_points,
            }
        };
        disks.push(disk);
    }

    Ok(HardwareInfo { mainboard, disks })
}

impl DiskInterface {
    fn from_native(value: u32) -> Self {
        match value {
            0 => Self::Nvme,
            1 => Self::Usb,
            2 => Self::Usb1,
            3 => Self::Usb2,
            4 => Self::Usb3FiveGbit,
            5 => Self::Usb3TenGbit,
            6 => Self::Usb3TwentyGbit,
            7 => Self::Usb4TwentyGbit,
            8 => Self::Usb4FortyGbit,
            9 => Self::Usb4EightyGbit,
            10 => Self::Sata,
            11 => Self::Scsi,
            _ => Self::Unknown,
        }
    }
}

unsafe fn string_from_ptr(pointer: *const c_char) -> Option<String> {
    // SAFETY: callers guarantee that non-null pointers refer to valid,
    // nul-terminated strings for the duration of this call.
    let value = unsafe { raw_string_from_ptr(pointer) }?;
    if value.is_empty() || value.eq_ignore_ascii_case("<unknown>") {
        None
    } else {
        Some(value)
    }
}

unsafe fn raw_string_from_ptr(pointer: *const c_char) -> Option<String> {
    if pointer.is_null() {
        return None;
    }
    // SAFETY: upheld by this function's caller.
    Some(
        unsafe { CStr::from_ptr(pointer) }
            .to_string_lossy()
            .trim()
            .to_owned(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_interface_values_are_mapped() {
        assert_eq!(DiskInterface::from_native(0), DiskInterface::Nvme);
        assert_eq!(DiskInterface::from_native(10), DiskInterface::Sata);
        assert_eq!(DiskInterface::from_native(u32::MAX), DiskInterface::Unknown);
    }

    #[test]
    fn collection_smoke_test() {
        collect().expect("native hardware collection should not fail");
    }
}
