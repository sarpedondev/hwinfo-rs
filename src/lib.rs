//! Safe, owned Rust snapshots of the hardware information exposed by
//! [`lfreist/hwinfo`](https://github.com/lfreist/hwinfo).
//!
//! Native C++ objects and exceptions are contained behind a small C ABI. The
//! aggregate [`collect`] function preserves data from components that succeed
//! and reports failures through [`HardwareInfo::errors`].

use std::{
    error::Error as StdError,
    ffi::{CStr, c_char},
    fmt,
    ptr::NonNull,
};

#[cfg(feature = "monitoring")]
use std::{ffi::CString, time::Duration};

mod sys {
    #![allow(dead_code)]
    use std::ffi::c_char;
    #[repr(C)]
    pub struct Snapshot {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct CpuMetrics {
        _private: [u8; 0],
    }
    unsafe extern "C" {
        pub fn hwinfo_rs_last_error() -> *const c_char;
        pub fn hwinfo_rs_snapshot_collect(mask: u32) -> *mut Snapshot;
        pub fn hwinfo_rs_snapshot_free(v: *mut Snapshot);
        pub fn hwinfo_rs_snapshot_error_count(v: *const Snapshot) -> usize;
        pub fn hwinfo_rs_snapshot_error_component(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_snapshot_error_message(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_cpu_count(v: *const Snapshot) -> usize;
        pub fn hwinfo_rs_cpu_id(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_cpu_model(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_cpu_vendor(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_cpu_physical_cores(v: *const Snapshot, i: usize) -> u64;
        pub fn hwinfo_rs_cpu_logical_cores(v: *const Snapshot, i: usize) -> u64;
        pub fn hwinfo_rs_cpu_flag_count(v: *const Snapshot, i: usize) -> usize;
        pub fn hwinfo_rs_cpu_flag(v: *const Snapshot, i: usize, j: usize) -> *const c_char;
        pub fn hwinfo_rs_cpu_core_count(v: *const Snapshot, i: usize) -> usize;
        pub fn hwinfo_rs_cpu_core_id(v: *const Snapshot, i: usize, j: usize) -> u64;
        pub fn hwinfo_rs_cpu_core_l1_data(v: *const Snapshot, i: usize, j: usize) -> u64;
        pub fn hwinfo_rs_cpu_core_l1_instruction(v: *const Snapshot, i: usize, j: usize) -> u64;
        pub fn hwinfo_rs_cpu_core_l2(v: *const Snapshot, i: usize, j: usize) -> u64;
        pub fn hwinfo_rs_cpu_core_l3(v: *const Snapshot, i: usize, j: usize) -> u64;
        pub fn hwinfo_rs_cpu_core_regular_frequency(v: *const Snapshot, i: usize, j: usize) -> u64;
        pub fn hwinfo_rs_cpu_core_max_frequency(v: *const Snapshot, i: usize, j: usize) -> u64;
        pub fn hwinfo_rs_cpu_core_smt(v: *const Snapshot, i: usize, j: usize) -> u8;
        pub fn hwinfo_rs_gpu_count(v: *const Snapshot) -> usize;
        pub fn hwinfo_rs_gpu_id(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_gpu_vendor(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_gpu_name(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_gpu_driver_version(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_gpu_vendor_id(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_gpu_device_id(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_gpu_dedicated_memory(v: *const Snapshot, i: usize) -> u64;
        pub fn hwinfo_rs_gpu_shared_memory(v: *const Snapshot, i: usize) -> u64;
        pub fn hwinfo_rs_gpu_frequency(v: *const Snapshot, i: usize) -> u64;
        pub fn hwinfo_rs_gpu_core_count(v: *const Snapshot, i: usize) -> u64;
        pub fn hwinfo_rs_memory_present(v: *const Snapshot) -> u8;
        pub fn hwinfo_rs_memory_size(v: *const Snapshot) -> u64;
        pub fn hwinfo_rs_memory_free(v: *const Snapshot) -> u64;
        pub fn hwinfo_rs_memory_available(v: *const Snapshot) -> u64;
        pub fn hwinfo_rs_memory_module_count(v: *const Snapshot) -> usize;
        pub fn hwinfo_rs_memory_module_id(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_memory_module_vendor(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_memory_module_name(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_memory_module_model(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_memory_module_serial(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_memory_module_size(v: *const Snapshot, i: usize) -> u64;
        pub fn hwinfo_rs_memory_module_frequency(v: *const Snapshot, i: usize) -> u64;
        pub fn hwinfo_rs_os_present(v: *const Snapshot) -> u8;
        pub fn hwinfo_rs_os_name(v: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_os_version(v: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_os_kernel(v: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_os_is_32_bit(v: *const Snapshot) -> u8;
        pub fn hwinfo_rs_os_is_64_bit(v: *const Snapshot) -> u8;
        pub fn hwinfo_rs_os_is_big_endian(v: *const Snapshot) -> u8;
        pub fn hwinfo_rs_os_is_little_endian(v: *const Snapshot) -> u8;
        pub fn hwinfo_rs_battery_count(v: *const Snapshot) -> usize;
        pub fn hwinfo_rs_battery_id(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_battery_vendor(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_battery_model(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_battery_serial(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_battery_technology(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_battery_energy_full(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_battery_energy_now(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_battery_capacity(v: *const Snapshot, i: usize) -> f64;
        pub fn hwinfo_rs_battery_state(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_network_count(v: *const Snapshot) -> usize;
        pub fn hwinfo_rs_network_index(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_network_description(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_network_mac(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_network_ipv4(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_network_ipv6(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_mainboard_present(v: *const Snapshot) -> u8;
        pub fn hwinfo_rs_mainboard_vendor(v: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_mainboard_name(v: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_mainboard_version(v: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_mainboard_serial(v: *const Snapshot) -> *const c_char;
        pub fn hwinfo_rs_disk_count(v: *const Snapshot) -> usize;
        pub fn hwinfo_rs_disk_id(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_disk_vendor(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_disk_model(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_disk_serial(v: *const Snapshot, i: usize) -> *const c_char;
        pub fn hwinfo_rs_disk_size_bytes(v: *const Snapshot, i: usize) -> u64;
        pub fn hwinfo_rs_disk_interface(v: *const Snapshot, i: usize) -> u32;
        pub fn hwinfo_rs_disk_mount_point_count(v: *const Snapshot, i: usize) -> usize;
        pub fn hwinfo_rs_disk_mount_point(v: *const Snapshot, i: usize, j: usize) -> *const c_char;
        pub fn hwinfo_rs_cpu_metrics_collect(ms: u64) -> *mut CpuMetrics;
        pub fn hwinfo_rs_cpu_metrics_free(v: *mut CpuMetrics);
        pub fn hwinfo_rs_cpu_metrics_utilization(v: *const CpuMetrics) -> f64;
        pub fn hwinfo_rs_cpu_metrics_thread_count(v: *const CpuMetrics) -> usize;
        pub fn hwinfo_rs_cpu_metrics_thread_utilization(v: *const CpuMetrics, i: usize) -> f64;
        pub fn hwinfo_rs_cpu_metrics_thread_frequency(v: *const CpuMetrics, i: usize) -> i64;
        pub fn hwinfo_rs_memory_metrics_collect(free: *mut u64, available: *mut u64) -> u8;
        pub fn hwinfo_rs_disk_metrics_collect(mount: *const c_char, free: *mut u64) -> u8;
    }
}

#[cfg(feature = "cpu")]
const CPU: u32 = 1 << 0;
#[cfg(feature = "gpu")]
const GPU: u32 = 1 << 1;
#[cfg(feature = "memory")]
const MEMORY: u32 = 1 << 2;
#[cfg(feature = "os")]
const OS: u32 = 1 << 3;
#[cfg(feature = "battery")]
const BATTERY: u32 = 1 << 4;
#[cfg(feature = "network")]
const NETWORK: u32 = 1 << 5;
#[cfg(feature = "mainboard")]
const MAINBOARD: u32 = 1 << 6;
#[cfg(feature = "disk")]
const DISK: u32 = 1 << 7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Component {
    Cpu,
    Gpu,
    Memory,
    Os,
    Battery,
    Network,
    Mainboard,
    Disk,
    Unknown(u32),
}
impl Component {
    fn from_native(v: u32) -> Self {
        match v {
            0 => Self::Cpu,
            1 => Self::Gpu,
            2 => Self::Memory,
            3 => Self::Os,
            4 => Self::Battery,
            5 => Self::Network,
            6 => Self::Mainboard,
            7 => Self::Disk,
            x => Self::Unknown(x),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentError {
    pub component: Component,
    message: String,
}
impl ComponentError {
    pub fn message(&self) -> &str {
        &self.message
    }
}
impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} collection failed: {}",
            self.component, self.message
        )
    }
}
impl StdError for ComponentError {}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionError {
    message: String,
}
impl CollectionError {
    pub fn message(&self) -> &str {
        &self.message
    }
    fn native() -> Self {
        Self {
            message: last_error(),
        }
    }
}
impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hardware collection failed: {}", self.message)
    }
}
impl StdError for CollectionError {}

struct NativeSnapshot(NonNull<sys::Snapshot>);
impl NativeSnapshot {
    fn collect(mask: u32) -> Result<Self, CollectionError> {
        NonNull::new(unsafe { sys::hwinfo_rs_snapshot_collect(mask) })
            .map(Self)
            .ok_or_else(CollectionError::native)
    }
    fn ptr(&self) -> *const sys::Snapshot {
        self.0.as_ptr()
    }
    fn errors(&self) -> Vec<ComponentError> {
        unsafe {
            (0..sys::hwinfo_rs_snapshot_error_count(self.ptr()))
                .map(|i| ComponentError {
                    component: Component::from_native(sys::hwinfo_rs_snapshot_error_component(
                        self.ptr(),
                        i,
                    )),
                    message: raw_string(sys::hwinfo_rs_snapshot_error_message(self.ptr(), i))
                        .unwrap_or_else(|| "unknown native error".into()),
                })
                .collect()
        }
    }
}
impl Drop for NativeSnapshot {
    fn drop(&mut self) {
        unsafe { sys::hwinfo_rs_snapshot_free(self.0.as_ptr()) }
    }
}
#[cfg(any(feature = "memory", feature = "os", feature = "mainboard"))]
fn one<T>(
    mask: u32,
    component: Component,
    read: impl FnOnce(&NativeSnapshot) -> Option<T>,
) -> Result<T, ComponentError> {
    let native = NativeSnapshot::collect(mask).map_err(|e| ComponentError {
        component,
        message: e.message,
    })?;
    if let Some(error) = native
        .errors()
        .into_iter()
        .find(|e| e.component == component)
    {
        return Err(error);
    }
    read(&native).ok_or_else(|| ComponentError {
        component,
        message: "native collector returned no value".into(),
    })
}
#[cfg(any(
    feature = "cpu",
    feature = "gpu",
    feature = "battery",
    feature = "network",
    feature = "disk"
))]
fn many<T>(
    mask: u32,
    component: Component,
    read: impl FnOnce(&NativeSnapshot) -> Vec<T>,
) -> Result<Vec<T>, ComponentError> {
    let native = NativeSnapshot::collect(mask).map_err(|e| ComponentError {
        component,
        message: e.message,
    })?;
    if let Some(error) = native
        .errors()
        .into_iter()
        .find(|e| e.component == component)
    {
        return Err(error);
    }
    Ok(read(&native))
}

#[derive(Clone, Debug, PartialEq)]
pub struct HardwareInfo {
    #[cfg(feature = "cpu")]
    pub cpus: Vec<Cpu>,
    #[cfg(feature = "gpu")]
    pub gpus: Vec<Gpu>,
    #[cfg(feature = "memory")]
    pub memory: Option<MemoryInfo>,
    #[cfg(feature = "os")]
    pub os: Option<OperatingSystem>,
    #[cfg(feature = "battery")]
    pub batteries: Vec<Battery>,
    #[cfg(feature = "network")]
    pub networks: Vec<NetworkInterface>,
    #[cfg(feature = "mainboard")]
    pub mainboard: Option<Mainboard>,
    #[cfg(feature = "disk")]
    pub disks: Vec<Disk>,
    pub errors: Vec<ComponentError>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cpu {
    pub id: u32,
    pub model_name: Option<String>,
    pub vendor: Option<String>,
    pub physical_core_count: u64,
    pub logical_core_count: u64,
    pub flags: Vec<String>,
    pub cores: Vec<CpuCore>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CpuCore {
    pub id: u64,
    pub cache: CpuCache,
    pub regular_frequency_hz: u64,
    pub max_frequency_hz: u64,
    pub smt: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CpuCache {
    pub l1_data_bytes: u64,
    pub l1_instruction_bytes: u64,
    pub l2_bytes: u64,
    pub l3_bytes: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gpu {
    pub id: u32,
    pub vendor: Option<String>,
    pub name: Option<String>,
    pub driver_version: Option<String>,
    pub vendor_id: Option<String>,
    pub device_id: Option<String>,
    pub dedicated_memory_bytes: u64,
    pub shared_memory_bytes: u64,
    pub frequency_hz: u64,
    pub core_count: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryInfo {
    pub size_bytes: u64,
    pub free_bytes: u64,
    pub available_bytes: u64,
    pub modules: Vec<MemoryModule>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryModule {
    pub id: u32,
    pub vendor: Option<String>,
    pub name: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub size_bytes: u64,
    pub frequency_hz: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatingSystem {
    pub name: Option<String>,
    pub version: Option<String>,
    pub kernel: Option<String>,
    pub is_32_bit: bool,
    pub is_64_bit: bool,
    pub is_big_endian: bool,
    pub is_little_endian: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Battery {
    pub id: u32,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub technology: Option<String>,
    pub energy_full: u32,
    pub energy_now: u32,
    pub capacity: f64,
    pub state: BatteryState,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum BatteryState {
    Charging,
    Discharging,
    Unknown,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkInterface {
    pub interface_index: Option<String>,
    pub description: Option<String>,
    pub mac_address: Option<String>,
    pub ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
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
impl DiskInterface {
    #[cfg(any(feature = "disk", test))]
    fn from_native(v: u32) -> Self {
        match v {
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

unsafe fn raw_string(p: *const c_char) -> Option<String> {
    if p.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(p) }
                .to_string_lossy()
                .trim()
                .to_owned(),
        )
    }
}
#[cfg(any(
    feature = "cpu",
    feature = "gpu",
    feature = "memory",
    feature = "os",
    feature = "battery",
    feature = "network",
    feature = "mainboard",
    feature = "disk"
))]
unsafe fn string(p: *const c_char) -> Option<String> {
    let v = unsafe { raw_string(p) }?;
    if v.is_empty() || v.eq_ignore_ascii_case("<unknown>") {
        None
    } else {
        Some(v)
    }
}
fn last_error() -> String {
    unsafe { raw_string(sys::hwinfo_rs_last_error()) }
        .unwrap_or_else(|| "unknown native error".into())
}

#[cfg(feature = "cpu")]
unsafe fn read_cpus(p: *const sys::Snapshot) -> Vec<Cpu> {
    unsafe {
        (0..sys::hwinfo_rs_cpu_count(p))
            .map(|i| {
                let flags = (0..sys::hwinfo_rs_cpu_flag_count(p, i))
                    .filter_map(|j| raw_string(sys::hwinfo_rs_cpu_flag(p, i, j)))
                    .collect();
                let cores = (0..sys::hwinfo_rs_cpu_core_count(p, i))
                    .map(|j| CpuCore {
                        id: sys::hwinfo_rs_cpu_core_id(p, i, j),
                        cache: CpuCache {
                            l1_data_bytes: sys::hwinfo_rs_cpu_core_l1_data(p, i, j),
                            l1_instruction_bytes: sys::hwinfo_rs_cpu_core_l1_instruction(p, i, j),
                            l2_bytes: sys::hwinfo_rs_cpu_core_l2(p, i, j),
                            l3_bytes: sys::hwinfo_rs_cpu_core_l3(p, i, j),
                        },
                        regular_frequency_hz: sys::hwinfo_rs_cpu_core_regular_frequency(p, i, j),
                        max_frequency_hz: sys::hwinfo_rs_cpu_core_max_frequency(p, i, j),
                        smt: sys::hwinfo_rs_cpu_core_smt(p, i, j) != 0,
                    })
                    .collect();
                Cpu {
                    id: sys::hwinfo_rs_cpu_id(p, i),
                    model_name: string(sys::hwinfo_rs_cpu_model(p, i)),
                    vendor: string(sys::hwinfo_rs_cpu_vendor(p, i)),
                    physical_core_count: sys::hwinfo_rs_cpu_physical_cores(p, i),
                    logical_core_count: sys::hwinfo_rs_cpu_logical_cores(p, i),
                    flags,
                    cores,
                }
            })
            .collect()
    }
}
#[cfg(feature = "gpu")]
unsafe fn read_gpus(p: *const sys::Snapshot) -> Vec<Gpu> {
    unsafe {
        (0..sys::hwinfo_rs_gpu_count(p))
            .map(|i| Gpu {
                id: sys::hwinfo_rs_gpu_id(p, i),
                vendor: string(sys::hwinfo_rs_gpu_vendor(p, i)),
                name: string(sys::hwinfo_rs_gpu_name(p, i)),
                driver_version: string(sys::hwinfo_rs_gpu_driver_version(p, i)),
                vendor_id: string(sys::hwinfo_rs_gpu_vendor_id(p, i)),
                device_id: string(sys::hwinfo_rs_gpu_device_id(p, i)),
                dedicated_memory_bytes: sys::hwinfo_rs_gpu_dedicated_memory(p, i),
                shared_memory_bytes: sys::hwinfo_rs_gpu_shared_memory(p, i),
                frequency_hz: sys::hwinfo_rs_gpu_frequency(p, i),
                core_count: sys::hwinfo_rs_gpu_core_count(p, i),
            })
            .collect()
    }
}
#[cfg(feature = "memory")]
unsafe fn read_memory(p: *const sys::Snapshot) -> Option<MemoryInfo> {
    if unsafe { sys::hwinfo_rs_memory_present(p) } == 0 {
        return None;
    }
    Some(unsafe {
        MemoryInfo {
            size_bytes: sys::hwinfo_rs_memory_size(p),
            free_bytes: sys::hwinfo_rs_memory_free(p),
            available_bytes: sys::hwinfo_rs_memory_available(p),
            modules: (0..sys::hwinfo_rs_memory_module_count(p))
                .map(|i| MemoryModule {
                    id: sys::hwinfo_rs_memory_module_id(p, i),
                    vendor: string(sys::hwinfo_rs_memory_module_vendor(p, i)),
                    name: string(sys::hwinfo_rs_memory_module_name(p, i)),
                    model: string(sys::hwinfo_rs_memory_module_model(p, i)),
                    serial_number: string(sys::hwinfo_rs_memory_module_serial(p, i)),
                    size_bytes: sys::hwinfo_rs_memory_module_size(p, i),
                    frequency_hz: sys::hwinfo_rs_memory_module_frequency(p, i),
                })
                .collect(),
        }
    })
}
#[cfg(feature = "os")]
unsafe fn read_os(p: *const sys::Snapshot) -> Option<OperatingSystem> {
    if unsafe { sys::hwinfo_rs_os_present(p) } == 0 {
        return None;
    }
    Some(unsafe {
        OperatingSystem {
            name: string(sys::hwinfo_rs_os_name(p)),
            version: string(sys::hwinfo_rs_os_version(p)),
            kernel: string(sys::hwinfo_rs_os_kernel(p)),
            is_32_bit: sys::hwinfo_rs_os_is_32_bit(p) != 0,
            is_64_bit: sys::hwinfo_rs_os_is_64_bit(p) != 0,
            is_big_endian: sys::hwinfo_rs_os_is_big_endian(p) != 0,
            is_little_endian: sys::hwinfo_rs_os_is_little_endian(p) != 0,
        }
    })
}
#[cfg(feature = "battery")]
unsafe fn read_batteries(p: *const sys::Snapshot) -> Vec<Battery> {
    unsafe {
        (0..sys::hwinfo_rs_battery_count(p))
            .map(|i| Battery {
                id: sys::hwinfo_rs_battery_id(p, i),
                vendor: string(sys::hwinfo_rs_battery_vendor(p, i)),
                model: string(sys::hwinfo_rs_battery_model(p, i)),
                serial_number: string(sys::hwinfo_rs_battery_serial(p, i)),
                technology: string(sys::hwinfo_rs_battery_technology(p, i)),
                energy_full: sys::hwinfo_rs_battery_energy_full(p, i),
                energy_now: sys::hwinfo_rs_battery_energy_now(p, i),
                capacity: sys::hwinfo_rs_battery_capacity(p, i),
                state: match sys::hwinfo_rs_battery_state(p, i) {
                    0 => BatteryState::Charging,
                    1 => BatteryState::Discharging,
                    _ => BatteryState::Unknown,
                },
            })
            .collect()
    }
}
#[cfg(feature = "network")]
unsafe fn read_networks(p: *const sys::Snapshot) -> Vec<NetworkInterface> {
    unsafe {
        (0..sys::hwinfo_rs_network_count(p))
            .map(|i| NetworkInterface {
                interface_index: string(sys::hwinfo_rs_network_index(p, i)),
                description: string(sys::hwinfo_rs_network_description(p, i)),
                mac_address: string(sys::hwinfo_rs_network_mac(p, i)),
                ipv4_address: string(sys::hwinfo_rs_network_ipv4(p, i)),
                ipv6_address: string(sys::hwinfo_rs_network_ipv6(p, i)),
            })
            .collect()
    }
}
#[cfg(feature = "mainboard")]
unsafe fn read_mainboard(p: *const sys::Snapshot) -> Option<Mainboard> {
    if unsafe { sys::hwinfo_rs_mainboard_present(p) } == 0 {
        return None;
    }
    Some(unsafe {
        Mainboard {
            vendor: string(sys::hwinfo_rs_mainboard_vendor(p)),
            name: string(sys::hwinfo_rs_mainboard_name(p)),
            version: string(sys::hwinfo_rs_mainboard_version(p)),
            serial_number: string(sys::hwinfo_rs_mainboard_serial(p)),
        }
    })
}
#[cfg(feature = "disk")]
unsafe fn read_disks(p: *const sys::Snapshot) -> Vec<Disk> {
    unsafe {
        (0..sys::hwinfo_rs_disk_count(p))
            .map(|i| Disk {
                id: sys::hwinfo_rs_disk_id(p, i),
                vendor: string(sys::hwinfo_rs_disk_vendor(p, i)),
                model: string(sys::hwinfo_rs_disk_model(p, i)),
                serial_number: string(sys::hwinfo_rs_disk_serial(p, i)),
                size_bytes: sys::hwinfo_rs_disk_size_bytes(p, i),
                interface: DiskInterface::from_native(sys::hwinfo_rs_disk_interface(p, i)),
                mount_points: (0..sys::hwinfo_rs_disk_mount_point_count(p, i))
                    .filter_map(|j| raw_string(sys::hwinfo_rs_disk_mount_point(p, i, j)))
                    .collect(),
            })
            .collect()
    }
}

/// Collects every component enabled at compile time.
pub fn collect() -> Result<HardwareInfo, CollectionError> {
    #[allow(unused_mut)]
    let mut mask = 0;
    #[cfg(feature = "cpu")]
    {
        mask |= CPU
    }
    #[cfg(feature = "gpu")]
    {
        mask |= GPU
    }
    #[cfg(feature = "memory")]
    {
        mask |= MEMORY
    }
    #[cfg(feature = "os")]
    {
        mask |= OS
    }
    #[cfg(feature = "battery")]
    {
        mask |= BATTERY
    }
    #[cfg(feature = "network")]
    {
        mask |= NETWORK
    }
    #[cfg(feature = "mainboard")]
    {
        mask |= MAINBOARD
    }
    #[cfg(feature = "disk")]
    {
        mask |= DISK
    }
    let n = NativeSnapshot::collect(mask)?;
    #[allow(unused_variables)]
    let p = n.ptr();
    Ok(HardwareInfo {
        #[cfg(feature = "cpu")]
        cpus: unsafe { read_cpus(p) },
        #[cfg(feature = "gpu")]
        gpus: unsafe { read_gpus(p) },
        #[cfg(feature = "memory")]
        memory: unsafe { read_memory(p) },
        #[cfg(feature = "os")]
        os: unsafe { read_os(p) },
        #[cfg(feature = "battery")]
        batteries: unsafe { read_batteries(p) },
        #[cfg(feature = "network")]
        networks: unsafe { read_networks(p) },
        #[cfg(feature = "mainboard")]
        mainboard: unsafe { read_mainboard(p) },
        #[cfg(feature = "disk")]
        disks: unsafe { read_disks(p) },
        errors: n.errors(),
    })
}

macro_rules! module_many {
    ($mod:ident,$feature:literal,$mask:ident,$component:ident,$ty:ty,$read:ident) => {
        #[cfg(feature=$feature)]
        pub mod $mod {
            use super::*;
            pub fn collect() -> Result<Vec<$ty>, ComponentError> {
                many($mask, Component::$component, |n| unsafe { $read(n.ptr()) })
            }
        }
    };
}
module_many!(cpu, "cpu", CPU, Cpu, Cpu, read_cpus);
module_many!(gpu, "gpu", GPU, Gpu, Gpu, read_gpus);
module_many!(
    battery,
    "battery",
    BATTERY,
    Battery,
    Battery,
    read_batteries
);
module_many!(
    network,
    "network",
    NETWORK,
    Network,
    NetworkInterface,
    read_networks
);
module_many!(disk, "disk", DISK, Disk, Disk, read_disks);
#[cfg(feature = "memory")]
pub mod memory {
    use super::*;
    pub fn collect() -> Result<MemoryInfo, ComponentError> {
        one(MEMORY, Component::Memory, |n| unsafe {
            read_memory(n.ptr())
        })
    }
}
#[cfg(feature = "os")]
pub mod os {
    use super::*;
    pub fn collect() -> Result<OperatingSystem, ComponentError> {
        one(OS, Component::Os, |n| unsafe { read_os(n.ptr()) })
    }
}
#[cfg(feature = "mainboard")]
pub mod mainboard {
    use super::*;
    pub fn collect() -> Result<Mainboard, ComponentError> {
        one(MAINBOARD, Component::Mainboard, |n| unsafe {
            read_mainboard(n.ptr())
        })
    }
}

#[cfg(feature = "monitoring")]
pub mod monitoring {
    use super::*;
    #[derive(Clone, Debug, PartialEq)]
    pub struct CpuMetrics {
        pub utilization: f64,
        pub thread_utilization: Vec<f64>,
        pub thread_frequency_hz: Vec<i64>,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct MemoryMetrics {
        pub free_bytes: u64,
        pub available_bytes: u64,
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct DiskMetrics {
        pub mount_point: String,
        pub free_bytes: u64,
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct MonitoringError {
        message: String,
    }
    impl MonitoringError {
        pub fn message(&self) -> &str {
            &self.message
        }
    }
    impl fmt::Display for MonitoringError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "monitoring snapshot failed: {}", self.message)
        }
    }
    impl StdError for MonitoringError {}
    struct NativeCpuMetrics(NonNull<sys::CpuMetrics>);
    impl Drop for NativeCpuMetrics {
        fn drop(&mut self) {
            unsafe { sys::hwinfo_rs_cpu_metrics_free(self.0.as_ptr()) }
        }
    }
    pub fn cpu(sample_duration: Duration) -> Result<CpuMetrics, MonitoringError> {
        let ms = u64::try_from(sample_duration.as_millis()).map_err(|_| MonitoringError {
            message: "sample duration is too large".into(),
        })?;
        if ms == 0 {
            return Err(MonitoringError {
                message: "sample duration must be at least one millisecond".into(),
            });
        }
        let n = NativeCpuMetrics(
            NonNull::new(unsafe { sys::hwinfo_rs_cpu_metrics_collect(ms) }).ok_or_else(|| {
                MonitoringError {
                    message: last_error(),
                }
            })?,
        );
        let p = n.0.as_ptr();
        let count = unsafe { sys::hwinfo_rs_cpu_metrics_thread_count(p) };
        Ok(CpuMetrics {
            utilization: unsafe { sys::hwinfo_rs_cpu_metrics_utilization(p) },
            thread_utilization: (0..count)
                .map(|i| unsafe { sys::hwinfo_rs_cpu_metrics_thread_utilization(p, i) })
                .collect(),
            thread_frequency_hz: (0..count)
                .map(|i| unsafe { sys::hwinfo_rs_cpu_metrics_thread_frequency(p, i) })
                .collect(),
        })
    }
    pub fn memory() -> Result<MemoryMetrics, MonitoringError> {
        let (mut free, mut available) = (0, 0);
        if unsafe { sys::hwinfo_rs_memory_metrics_collect(&mut free, &mut available) } == 0 {
            return Err(MonitoringError {
                message: last_error(),
            });
        }
        Ok(MemoryMetrics {
            free_bytes: free,
            available_bytes: available,
        })
    }
    pub fn disk(mount_point: &str) -> Result<DiskMetrics, MonitoringError> {
        let path = CString::new(mount_point).map_err(|_| MonitoringError {
            message: "mount point contains a NUL byte".into(),
        })?;
        let mut free = 0;
        if unsafe { sys::hwinfo_rs_disk_metrics_collect(path.as_ptr(), &mut free) } == 0 {
            return Err(MonitoringError {
                message: last_error(),
            });
        }
        Ok(DiskMetrics {
            mount_point: mount_point.into(),
            free_bytes: free,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enum_fallbacks() {
        assert_eq!(DiskInterface::from_native(0), DiskInterface::Nvme);
        assert_eq!(DiskInterface::from_native(u32::MAX), DiskInterface::Unknown);
        assert_eq!(Component::from_native(99), Component::Unknown(99));
    }
    #[test]
    fn aggregate_smoke() {
        collect().expect("aggregate snapshot allocation should succeed");
    }
    #[cfg(feature = "full")]
    #[test]
    fn component_collectors_smoke() {
        cpu::collect().expect("CPU collection should be contained");
        gpu::collect().expect("GPU collection should be contained");
        memory::collect().expect("memory collection should be contained");
        os::collect().expect("OS collection should be contained");
        battery::collect().expect("battery collection should be contained");
        network::collect().expect("network collection should be contained");
        mainboard::collect().expect("mainboard collection should be contained");
        disk::collect().expect("disk collection should be contained");
    }
    #[cfg(feature = "monitoring")]
    #[test]
    fn rejects_zero_sample() {
        assert!(monitoring::cpu(Duration::ZERO).is_err());
    }
    #[cfg(all(feature = "monitoring", target_os = "linux"))]
    #[test]
    fn monitoring_smoke() {
        monitoring::cpu(Duration::from_millis(1)).expect("CPU metrics should be collected");
        monitoring::memory().expect("memory metrics should be collected");
        monitoring::disk("/").expect("root filesystem metrics should be collected");
    }
}
