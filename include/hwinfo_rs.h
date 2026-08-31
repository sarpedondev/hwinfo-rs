#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct hwinfo_rs_snapshot hwinfo_rs_snapshot;

typedef enum hwinfo_rs_disk_interface_code {
  HWINFO_RS_DISK_NVME = 0,
  HWINFO_RS_DISK_USB = 1,
  HWINFO_RS_DISK_USB1 = 2,
  HWINFO_RS_DISK_USB2 = 3,
  HWINFO_RS_DISK_USB3_5GBIT = 4,
  HWINFO_RS_DISK_USB3_10GBIT = 5,
  HWINFO_RS_DISK_USB3_20GBIT = 6,
  HWINFO_RS_DISK_USB4_20GBIT = 7,
  HWINFO_RS_DISK_USB4_40GBIT = 8,
  HWINFO_RS_DISK_USB4_80GBIT = 9,
  HWINFO_RS_DISK_SATA = 10,
  HWINFO_RS_DISK_SCSI = 11,
  HWINFO_RS_DISK_UNKNOWN = 255,
} hwinfo_rs_disk_interface_code;

const char* hwinfo_rs_last_error(void);

hwinfo_rs_snapshot* hwinfo_rs_snapshot_collect(void);
void hwinfo_rs_snapshot_free(hwinfo_rs_snapshot* snapshot);

const char* hwinfo_rs_mainboard_vendor(const hwinfo_rs_snapshot* snapshot);
const char* hwinfo_rs_mainboard_name(const hwinfo_rs_snapshot* snapshot);
const char* hwinfo_rs_mainboard_version(const hwinfo_rs_snapshot* snapshot);
const char* hwinfo_rs_mainboard_serial(const hwinfo_rs_snapshot* snapshot);

size_t hwinfo_rs_disk_count(const hwinfo_rs_snapshot* snapshot);
uint32_t hwinfo_rs_disk_id(const hwinfo_rs_snapshot* snapshot, size_t index);
const char* hwinfo_rs_disk_vendor(const hwinfo_rs_snapshot* snapshot, size_t index);
const char* hwinfo_rs_disk_model(const hwinfo_rs_snapshot* snapshot, size_t index);
const char* hwinfo_rs_disk_serial(const hwinfo_rs_snapshot* snapshot, size_t index);
uint64_t hwinfo_rs_disk_size_bytes(const hwinfo_rs_snapshot* snapshot, size_t index);
uint32_t hwinfo_rs_disk_interface(const hwinfo_rs_snapshot* snapshot, size_t index);
size_t hwinfo_rs_disk_mount_point_count(const hwinfo_rs_snapshot* snapshot, size_t index);
const char* hwinfo_rs_disk_mount_point(const hwinfo_rs_snapshot* snapshot, size_t disk_index, size_t mount_index);

#ifdef __cplusplus
}
#endif
