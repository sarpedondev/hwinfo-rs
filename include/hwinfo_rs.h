#pragma once
#include <stddef.h>
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
typedef struct hwinfo_rs_snapshot hwinfo_rs_snapshot;
typedef struct hwinfo_rs_cpu_metrics hwinfo_rs_cpu_metrics;
enum hwinfo_rs_component { HWINFO_RS_COMPONENT_CPU, HWINFO_RS_COMPONENT_GPU, HWINFO_RS_COMPONENT_MEMORY, HWINFO_RS_COMPONENT_OS, HWINFO_RS_COMPONENT_BATTERY, HWINFO_RS_COMPONENT_NETWORK, HWINFO_RS_COMPONENT_MAINBOARD, HWINFO_RS_COMPONENT_DISK };
enum hwinfo_rs_component_mask { HWINFO_RS_MASK_CPU=1u<<0, HWINFO_RS_MASK_GPU=1u<<1, HWINFO_RS_MASK_MEMORY=1u<<2, HWINFO_RS_MASK_OS=1u<<3, HWINFO_RS_MASK_BATTERY=1u<<4, HWINFO_RS_MASK_NETWORK=1u<<5, HWINFO_RS_MASK_MAINBOARD=1u<<6, HWINFO_RS_MASK_DISK=1u<<7 };
const char* hwinfo_rs_last_error(void);
hwinfo_rs_snapshot* hwinfo_rs_snapshot_collect(uint32_t);
void hwinfo_rs_snapshot_free(hwinfo_rs_snapshot*);
size_t hwinfo_rs_snapshot_error_count(const hwinfo_rs_snapshot*);
uint32_t hwinfo_rs_snapshot_error_component(const hwinfo_rs_snapshot*,size_t);
const char* hwinfo_rs_snapshot_error_message(const hwinfo_rs_snapshot*,size_t);
size_t hwinfo_rs_cpu_count(const hwinfo_rs_snapshot*);
uint32_t hwinfo_rs_cpu_id(const hwinfo_rs_snapshot*,size_t);
const char* hwinfo_rs_cpu_model(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_cpu_vendor(const hwinfo_rs_snapshot*,size_t);
uint64_t hwinfo_rs_cpu_physical_cores(const hwinfo_rs_snapshot*,size_t); uint64_t hwinfo_rs_cpu_logical_cores(const hwinfo_rs_snapshot*,size_t);
size_t hwinfo_rs_cpu_flag_count(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_cpu_flag(const hwinfo_rs_snapshot*,size_t,size_t);
size_t hwinfo_rs_cpu_core_count(const hwinfo_rs_snapshot*,size_t); uint64_t hwinfo_rs_cpu_core_id(const hwinfo_rs_snapshot*,size_t,size_t);
uint64_t hwinfo_rs_cpu_core_l1_data(const hwinfo_rs_snapshot*,size_t,size_t); uint64_t hwinfo_rs_cpu_core_l1_instruction(const hwinfo_rs_snapshot*,size_t,size_t);
uint64_t hwinfo_rs_cpu_core_l2(const hwinfo_rs_snapshot*,size_t,size_t); uint64_t hwinfo_rs_cpu_core_l3(const hwinfo_rs_snapshot*,size_t,size_t);
uint64_t hwinfo_rs_cpu_core_regular_frequency(const hwinfo_rs_snapshot*,size_t,size_t); uint64_t hwinfo_rs_cpu_core_max_frequency(const hwinfo_rs_snapshot*,size_t,size_t); uint8_t hwinfo_rs_cpu_core_smt(const hwinfo_rs_snapshot*,size_t,size_t);
size_t hwinfo_rs_gpu_count(const hwinfo_rs_snapshot*); uint32_t hwinfo_rs_gpu_id(const hwinfo_rs_snapshot*,size_t);
const char* hwinfo_rs_gpu_vendor(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_gpu_name(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_gpu_driver_version(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_gpu_vendor_id(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_gpu_device_id(const hwinfo_rs_snapshot*,size_t);
uint64_t hwinfo_rs_gpu_dedicated_memory(const hwinfo_rs_snapshot*,size_t); uint64_t hwinfo_rs_gpu_shared_memory(const hwinfo_rs_snapshot*,size_t); uint64_t hwinfo_rs_gpu_frequency(const hwinfo_rs_snapshot*,size_t); uint64_t hwinfo_rs_gpu_core_count(const hwinfo_rs_snapshot*,size_t);
uint8_t hwinfo_rs_memory_present(const hwinfo_rs_snapshot*); uint64_t hwinfo_rs_memory_size(const hwinfo_rs_snapshot*); uint64_t hwinfo_rs_memory_free(const hwinfo_rs_snapshot*); uint64_t hwinfo_rs_memory_available(const hwinfo_rs_snapshot*);
size_t hwinfo_rs_memory_module_count(const hwinfo_rs_snapshot*); uint32_t hwinfo_rs_memory_module_id(const hwinfo_rs_snapshot*,size_t);
const char* hwinfo_rs_memory_module_vendor(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_memory_module_name(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_memory_module_model(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_memory_module_serial(const hwinfo_rs_snapshot*,size_t);
uint64_t hwinfo_rs_memory_module_size(const hwinfo_rs_snapshot*,size_t); uint64_t hwinfo_rs_memory_module_frequency(const hwinfo_rs_snapshot*,size_t);
uint8_t hwinfo_rs_os_present(const hwinfo_rs_snapshot*); const char* hwinfo_rs_os_name(const hwinfo_rs_snapshot*); const char* hwinfo_rs_os_version(const hwinfo_rs_snapshot*); const char* hwinfo_rs_os_kernel(const hwinfo_rs_snapshot*);
uint8_t hwinfo_rs_os_is_32_bit(const hwinfo_rs_snapshot*); uint8_t hwinfo_rs_os_is_64_bit(const hwinfo_rs_snapshot*); uint8_t hwinfo_rs_os_is_big_endian(const hwinfo_rs_snapshot*); uint8_t hwinfo_rs_os_is_little_endian(const hwinfo_rs_snapshot*);
size_t hwinfo_rs_battery_count(const hwinfo_rs_snapshot*); uint32_t hwinfo_rs_battery_id(const hwinfo_rs_snapshot*,size_t);
const char* hwinfo_rs_battery_vendor(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_battery_model(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_battery_serial(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_battery_technology(const hwinfo_rs_snapshot*,size_t);
uint32_t hwinfo_rs_battery_energy_full(const hwinfo_rs_snapshot*,size_t); uint32_t hwinfo_rs_battery_energy_now(const hwinfo_rs_snapshot*,size_t); double hwinfo_rs_battery_capacity(const hwinfo_rs_snapshot*,size_t); uint32_t hwinfo_rs_battery_state(const hwinfo_rs_snapshot*,size_t);
size_t hwinfo_rs_network_count(const hwinfo_rs_snapshot*); const char* hwinfo_rs_network_index(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_network_description(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_network_mac(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_network_ipv4(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_network_ipv6(const hwinfo_rs_snapshot*,size_t);
uint8_t hwinfo_rs_mainboard_present(const hwinfo_rs_snapshot*); const char* hwinfo_rs_mainboard_vendor(const hwinfo_rs_snapshot*); const char* hwinfo_rs_mainboard_name(const hwinfo_rs_snapshot*); const char* hwinfo_rs_mainboard_version(const hwinfo_rs_snapshot*); const char* hwinfo_rs_mainboard_serial(const hwinfo_rs_snapshot*);
size_t hwinfo_rs_disk_count(const hwinfo_rs_snapshot*); uint32_t hwinfo_rs_disk_id(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_disk_vendor(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_disk_model(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_disk_serial(const hwinfo_rs_snapshot*,size_t); uint64_t hwinfo_rs_disk_size_bytes(const hwinfo_rs_snapshot*,size_t); uint32_t hwinfo_rs_disk_interface(const hwinfo_rs_snapshot*,size_t); size_t hwinfo_rs_disk_mount_point_count(const hwinfo_rs_snapshot*,size_t); const char* hwinfo_rs_disk_mount_point(const hwinfo_rs_snapshot*,size_t,size_t);
hwinfo_rs_cpu_metrics* hwinfo_rs_cpu_metrics_collect(uint64_t); void hwinfo_rs_cpu_metrics_free(hwinfo_rs_cpu_metrics*); double hwinfo_rs_cpu_metrics_utilization(const hwinfo_rs_cpu_metrics*); size_t hwinfo_rs_cpu_metrics_thread_count(const hwinfo_rs_cpu_metrics*); double hwinfo_rs_cpu_metrics_thread_utilization(const hwinfo_rs_cpu_metrics*,size_t); int64_t hwinfo_rs_cpu_metrics_thread_frequency(const hwinfo_rs_cpu_metrics*,size_t);
uint8_t hwinfo_rs_memory_metrics_collect(uint64_t*,uint64_t*); uint8_t hwinfo_rs_disk_metrics_collect(const char*,uint64_t*);
#ifdef __cplusplus
}
#endif
