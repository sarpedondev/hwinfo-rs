#include "hwinfo_rs.h"

#include <hwinfo/disk.h>
#include <hwinfo/mainboard.h>

#include <exception>
#include <new>
#include <string>
#include <utility>
#include <vector>

struct hwinfo_rs_snapshot {
  hwinfo::MainBoard mainboard;
  std::vector<hwinfo::Disk> disks;

  hwinfo_rs_snapshot() : mainboard(), disks(hwinfo::getAllDisks()) {}
};

namespace {

thread_local std::string last_error;

const hwinfo::Disk* disk_at(const hwinfo_rs_snapshot* snapshot, size_t index) noexcept {
  if (snapshot == nullptr || index >= snapshot->disks.size()) {
    return nullptr;
  }
  return &snapshot->disks[index];
}

uint32_t stable_disk_interface(hwinfo::Disk::Interface interface) noexcept {
  switch (interface) {
    case hwinfo::Disk::Interface::NVME:
      return HWINFO_RS_DISK_NVME;
    case hwinfo::Disk::Interface::USB:
      return HWINFO_RS_DISK_USB;
    case hwinfo::Disk::Interface::USB1:
      return HWINFO_RS_DISK_USB1;
    case hwinfo::Disk::Interface::USB2:
      return HWINFO_RS_DISK_USB2;
    case hwinfo::Disk::Interface::USB3_5GBit:
      return HWINFO_RS_DISK_USB3_5GBIT;
    case hwinfo::Disk::Interface::USB3_10GBit:
      return HWINFO_RS_DISK_USB3_10GBIT;
    case hwinfo::Disk::Interface::USB3_20GBit:
      return HWINFO_RS_DISK_USB3_20GBIT;
    case hwinfo::Disk::Interface::USB4_20GBit:
      return HWINFO_RS_DISK_USB4_20GBIT;
    case hwinfo::Disk::Interface::USB4_40GBit:
      return HWINFO_RS_DISK_USB4_40GBIT;
    case hwinfo::Disk::Interface::USB4_80GBit:
      return HWINFO_RS_DISK_USB4_80GBIT;
    case hwinfo::Disk::Interface::SATA:
      return HWINFO_RS_DISK_SATA;
    case hwinfo::Disk::Interface::SCSI:
      return HWINFO_RS_DISK_SCSI;
    case hwinfo::Disk::Interface::UNKNOWN:
      return HWINFO_RS_DISK_UNKNOWN;
  }
  return HWINFO_RS_DISK_UNKNOWN;
}

}  // namespace

extern "C" {

const char* hwinfo_rs_last_error(void) { return last_error.c_str(); }

hwinfo_rs_snapshot* hwinfo_rs_snapshot_collect(void) {
  try {
    last_error.clear();
    return new hwinfo_rs_snapshot();
  } catch (const std::exception& error) {
    last_error = error.what();
  } catch (...) {
    last_error = "unknown native exception";
  }
  return nullptr;
}

void hwinfo_rs_snapshot_free(hwinfo_rs_snapshot* snapshot) { delete snapshot; }

const char* hwinfo_rs_mainboard_vendor(const hwinfo_rs_snapshot* snapshot) {
  return snapshot == nullptr ? nullptr : snapshot->mainboard.vendor().c_str();
}

const char* hwinfo_rs_mainboard_name(const hwinfo_rs_snapshot* snapshot) {
  return snapshot == nullptr ? nullptr : snapshot->mainboard.name().c_str();
}

const char* hwinfo_rs_mainboard_version(const hwinfo_rs_snapshot* snapshot) {
  return snapshot == nullptr ? nullptr : snapshot->mainboard.version().c_str();
}

const char* hwinfo_rs_mainboard_serial(const hwinfo_rs_snapshot* snapshot) {
  return snapshot == nullptr ? nullptr : snapshot->mainboard.serialNumber().c_str();
}

size_t hwinfo_rs_disk_count(const hwinfo_rs_snapshot* snapshot) {
  return snapshot == nullptr ? 0 : snapshot->disks.size();
}

uint32_t hwinfo_rs_disk_id(const hwinfo_rs_snapshot* snapshot, size_t index) {
  const auto* disk = disk_at(snapshot, index);
  return disk == nullptr ? hwinfo::Disk::invalid_id : disk->id();
}

const char* hwinfo_rs_disk_vendor(const hwinfo_rs_snapshot* snapshot, size_t index) {
  const auto* disk = disk_at(snapshot, index);
  return disk == nullptr ? nullptr : disk->vendor().c_str();
}

const char* hwinfo_rs_disk_model(const hwinfo_rs_snapshot* snapshot, size_t index) {
  const auto* disk = disk_at(snapshot, index);
  return disk == nullptr ? nullptr : disk->model().c_str();
}

const char* hwinfo_rs_disk_serial(const hwinfo_rs_snapshot* snapshot, size_t index) {
  const auto* disk = disk_at(snapshot, index);
  return disk == nullptr ? nullptr : disk->serial_number().c_str();
}

uint64_t hwinfo_rs_disk_size_bytes(const hwinfo_rs_snapshot* snapshot, size_t index) {
  const auto* disk = disk_at(snapshot, index);
  return disk == nullptr ? 0 : disk->size();
}

uint32_t hwinfo_rs_disk_interface(const hwinfo_rs_snapshot* snapshot, size_t index) {
  const auto* disk = disk_at(snapshot, index);
  return disk == nullptr ? static_cast<uint32_t>(HWINFO_RS_DISK_UNKNOWN)
                         : stable_disk_interface(disk->disk_interface());
}

size_t hwinfo_rs_disk_mount_point_count(const hwinfo_rs_snapshot* snapshot, size_t index) {
  const auto* disk = disk_at(snapshot, index);
  return disk == nullptr ? 0 : disk->mount_points().size();
}

const char* hwinfo_rs_disk_mount_point(const hwinfo_rs_snapshot* snapshot, size_t disk_index, size_t mount_index) {
  const auto* disk = disk_at(snapshot, disk_index);
  if (disk == nullptr || mount_index >= disk->mount_points().size()) {
    return nullptr;
  }
  return disk->mount_points()[mount_index].c_str();
}

}  // extern "C"
