#include "hwinfo_rs.h"

#include <chrono>
#include <exception>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#ifdef HWINFO_RS_CPU
#include <hwinfo/cpu.h>
#endif
#ifdef HWINFO_RS_GPU
#include <hwinfo/gpu.h>
#endif
#ifdef HWINFO_RS_OPENCL
#include <hwinfo/opencl/device.h>
#endif
#ifdef HWINFO_RS_MEMORY
#include <hwinfo/ram.h>
#endif
#ifdef HWINFO_RS_OS
#include <hwinfo/os.h>
#endif
#ifdef HWINFO_RS_BATTERY
#include <hwinfo/battery.h>
#endif
#ifdef HWINFO_RS_NETWORK
#include <hwinfo/network.h>
#endif
#ifdef HWINFO_RS_MAINBOARD
#include <hwinfo/mainboard.h>
#endif
#ifdef HWINFO_RS_DISK
#include <hwinfo/disk.h>
#endif
#ifdef HWINFO_RS_MONITORING
#include <hwinfo/monitoring/cpu.h>
#include <hwinfo/monitoring/disk.h>
#include <hwinfo/monitoring/ram.h>
#endif

namespace {
struct Error { uint32_t component; std::string message; };
struct Core { uint64_t id, l1d, l1i, l2, l3, regular, maximum; bool smt; };
struct Cpu { uint32_t id; std::string model, vendor; uint64_t physical, logical; std::vector<std::string> flags; std::vector<Core> cores; };
struct Gpu { uint32_t id; std::string vendor, name, driver, vendor_id, device_id; uint64_t dedicated, shared, frequency, cores; };
struct Module { uint32_t id; std::string vendor, name, model, serial; uint64_t size, frequency; };
struct Memory { uint64_t size, free, available; std::vector<Module> modules; };
struct Os { std::string name, version, kernel; bool bit32, bit64, big, little; };
struct Battery { uint32_t id; std::string vendor, model, serial, technology; uint32_t full, now, state; double capacity; };
struct Network { std::string index, description, mac, ipv4, ipv6; };
struct Mainboard { std::string vendor, name, version, serial; };
struct Disk { uint32_t id, interface_code; std::string vendor, model, serial; uint64_t size; std::vector<std::string> mounts; };

thread_local std::string last_error;
template <class T> const T* at(const std::vector<T>& values, size_t i) { return i < values.size() ? &values[i] : nullptr; }
const char* str(const std::string* value) { return value == nullptr ? nullptr : value->c_str(); }

}

struct hwinfo_rs_snapshot {
  std::vector<Error> errors;
  std::vector<Cpu> cpus;
  std::vector<Gpu> gpus;
  std::unique_ptr<Memory> memory;
  std::unique_ptr<Os> os;
  std::vector<Battery> batteries;
  std::vector<Network> networks;
  std::unique_ptr<Mainboard> mainboard;
  std::vector<Disk> disks;
};

struct hwinfo_rs_cpu_metrics { double utilization; std::vector<double> threads; std::vector<int64_t> frequencies; };

namespace {
template <class F> void capture(hwinfo_rs_snapshot& snapshot, uint32_t component, F&& function) {
  try { function(); }
  catch (const std::exception& error) { snapshot.errors.push_back({component, error.what()}); }
  catch (...) { snapshot.errors.push_back({component, "unknown native exception"}); }
}

#ifdef HWINFO_RS_CPU
void collect_cpu(hwinfo_rs_snapshot& s) {
  for (const auto& value : hwinfo::getAllCPUs()) {
    Cpu cpu{value.id(), value.modelName(), value.vendor(), value.numPhysicalCores(), value.numLogicalCores(), value.flags(), {}};
    for (const auto& core : value.cores()) cpu.cores.push_back({core.id, core.cache.l1_data, core.cache.l1_instruction, core.cache.l2, core.cache.l3, core.regular_frequency_hz, core.max_frequency_hz, core.smt});
    s.cpus.push_back(std::move(cpu));
  }
}
#endif
#ifdef HWINFO_RS_GPU
void collect_gpu(hwinfo_rs_snapshot& s) {
  for (const auto& v : hwinfo::getAllGPUs()) s.gpus.push_back({v.id(),v.vendor(),v.name(),v.driverVersion(),v.vendor_id(),v.device_id(),v.dedicated_memory_Bytes(),v.shared_memory_Bytes(),v.frequency_hz(),v.num_cores()});
#ifdef HWINFO_RS_OPENCL
  const auto devices = opencl_::DeviceManager::get_list<opencl_::Filter::GPU>();
  for (auto& gpu : s.gpus) {
    for (const auto* device : devices) {
      const auto name = device->name();
      const auto vendor = device->vendor();
      const bool name_matches = gpu.name.empty() || name.find(gpu.name) != std::string::npos || gpu.name.find(name) != std::string::npos;
      const bool vendor_matches = gpu.vendor.empty() || vendor.find(gpu.vendor) != std::string::npos || gpu.vendor.find(vendor) != std::string::npos;
      if (name_matches && vendor_matches) {
        gpu.driver = device->driver_version();
        gpu.dedicated = device->memory_Bytes();
        gpu.frequency = device->clock_frequency_MHz() * 1000 * 1000;
        gpu.cores = device->cores();
        break;
      }
    }
  }
#endif
}
#endif
#ifdef HWINFO_RS_MEMORY
void collect_memory(hwinfo_rs_snapshot& s) { hwinfo::Memory v; auto m=std::make_unique<Memory>(); m->size=v.size(); m->free=v.free(); m->available=v.available(); for(const auto& x:v.modules())m->modules.push_back({x.id,x.vendor,x.name,x.model,x.serial_number,x._size_bytes,x.frequency_hz}); s.memory=std::move(m); }
#endif
#ifdef HWINFO_RS_OS
void collect_os(hwinfo_rs_snapshot& s) { hwinfo::OS v; s.os=std::make_unique<Os>(Os{v.name(),v.version(),v.kernel(),v.is32bit(),v.is64bit(),v.isBigEndian(),v.isLittleEndian()}); }
#endif
#ifdef HWINFO_RS_BATTERY
void collect_battery(hwinfo_rs_snapshot& s) { for(const auto& v:hwinfo::getAllBatteries()) s.batteries.push_back({v.id(),v.vendor(),v.model(),v.serialNumber(),v.technology(),v.energyFull(),v.energyNow(),static_cast<uint32_t>(v.state()),v.capacity()}); }
#endif
#ifdef HWINFO_RS_NETWORK
void collect_network(hwinfo_rs_snapshot& s) { for(const auto& v:hwinfo::getAllNetworks()) s.networks.push_back({v.interfaceIndex(),v.description(),v.mac(),v.ip4(),v.ip6()}); }
#endif
#ifdef HWINFO_RS_MAINBOARD
void collect_mainboard(hwinfo_rs_snapshot& s) { hwinfo::MainBoard v; s.mainboard=std::make_unique<Mainboard>(Mainboard{v.vendor(),v.name(),v.version(),v.serialNumber()}); }
#endif
#ifdef HWINFO_RS_DISK
uint32_t disk_interface(hwinfo::Disk::Interface v) { return static_cast<uint32_t>(v); }
void collect_disk(hwinfo_rs_snapshot& s) { for(const auto& v:hwinfo::getAllDisks()) s.disks.push_back({v.id(),disk_interface(v.disk_interface()),v.vendor(),v.model(),v.serial_number(),v.size(),v.mount_points()}); }
#endif
}

extern "C" {
const char* hwinfo_rs_last_error(void) { return last_error.c_str(); }
hwinfo_rs_snapshot* hwinfo_rs_snapshot_collect(uint32_t mask) {
  try {
    (void)mask;
    last_error.clear(); auto s=std::make_unique<hwinfo_rs_snapshot>();
#ifdef HWINFO_RS_CPU
    if(mask&HWINFO_RS_MASK_CPU) capture(*s,HWINFO_RS_COMPONENT_CPU,[&]{collect_cpu(*s);});
#endif
#ifdef HWINFO_RS_GPU
    if(mask&HWINFO_RS_MASK_GPU) capture(*s,HWINFO_RS_COMPONENT_GPU,[&]{collect_gpu(*s);});
#endif
#ifdef HWINFO_RS_MEMORY
    if(mask&HWINFO_RS_MASK_MEMORY) capture(*s,HWINFO_RS_COMPONENT_MEMORY,[&]{collect_memory(*s);});
#endif
#ifdef HWINFO_RS_OS
    if(mask&HWINFO_RS_MASK_OS) capture(*s,HWINFO_RS_COMPONENT_OS,[&]{collect_os(*s);});
#endif
#ifdef HWINFO_RS_BATTERY
    if(mask&HWINFO_RS_MASK_BATTERY) capture(*s,HWINFO_RS_COMPONENT_BATTERY,[&]{collect_battery(*s);});
#endif
#ifdef HWINFO_RS_NETWORK
    if(mask&HWINFO_RS_MASK_NETWORK) capture(*s,HWINFO_RS_COMPONENT_NETWORK,[&]{collect_network(*s);});
#endif
#ifdef HWINFO_RS_MAINBOARD
    if(mask&HWINFO_RS_MASK_MAINBOARD) capture(*s,HWINFO_RS_COMPONENT_MAINBOARD,[&]{collect_mainboard(*s);});
#endif
#ifdef HWINFO_RS_DISK
    if(mask&HWINFO_RS_MASK_DISK) capture(*s,HWINFO_RS_COMPONENT_DISK,[&]{collect_disk(*s);});
#endif
    return s.release();
  } catch(const std::exception& e){last_error=e.what();} catch(...){last_error="unknown native exception";} return nullptr;
}
void hwinfo_rs_snapshot_free(hwinfo_rs_snapshot* s){delete s;}
size_t hwinfo_rs_snapshot_error_count(const hwinfo_rs_snapshot*s){return s?s->errors.size():0;}
uint32_t hwinfo_rs_snapshot_error_component(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->errors,i):nullptr;return v?v->component:UINT32_MAX;}
const char* hwinfo_rs_snapshot_error_message(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->errors,i):nullptr;return v?v->message.c_str():nullptr;}

size_t hwinfo_rs_cpu_count(const hwinfo_rs_snapshot*s){return s?s->cpus.size():0;} uint32_t hwinfo_rs_cpu_id(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->cpus,i):nullptr;return v?v->id:UINT32_MAX;}
const char* hwinfo_rs_cpu_model(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->cpus,i):nullptr;return v?v->model.c_str():nullptr;} const char* hwinfo_rs_cpu_vendor(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->cpus,i):nullptr;return v?v->vendor.c_str():nullptr;}
uint64_t hwinfo_rs_cpu_physical_cores(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->cpus,i):nullptr;return v?v->physical:0;} uint64_t hwinfo_rs_cpu_logical_cores(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->cpus,i):nullptr;return v?v->logical:0;}
size_t hwinfo_rs_cpu_flag_count(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->cpus,i):nullptr;return v?v->flags.size():0;} const char* hwinfo_rs_cpu_flag(const hwinfo_rs_snapshot*s,size_t i,size_t j){auto v=s?at(s->cpus,i):nullptr;auto x=v?at(v->flags,j):nullptr;return str(x);}
size_t hwinfo_rs_cpu_core_count(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->cpus,i):nullptr;return v?v->cores.size():0;}
#define CORE_FN(name,field,type) type hwinfo_rs_cpu_core_##name(const hwinfo_rs_snapshot*s,size_t i,size_t j){auto c=s?at(s->cpus,i):nullptr;auto v=c?at(c->cores,j):nullptr;return v?v->field:0;}
CORE_FN(id,id,uint64_t) CORE_FN(l1_data,l1d,uint64_t) CORE_FN(l1_instruction,l1i,uint64_t) CORE_FN(l2,l2,uint64_t) CORE_FN(l3,l3,uint64_t) CORE_FN(regular_frequency,regular,uint64_t) CORE_FN(max_frequency,maximum,uint64_t) CORE_FN(smt,smt,uint8_t)

size_t hwinfo_rs_gpu_count(const hwinfo_rs_snapshot*s){return s?s->gpus.size():0;} uint32_t hwinfo_rs_gpu_id(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->gpus,i):nullptr;return v?v->id:UINT32_MAX;}
#define GPU_STR(name,field) const char* hwinfo_rs_gpu_##name(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->gpus,i):nullptr;return v?v->field.c_str():nullptr;}
GPU_STR(vendor,vendor) GPU_STR(name,name) GPU_STR(driver_version,driver) GPU_STR(vendor_id,vendor_id) GPU_STR(device_id,device_id)
#define GPU_NUM(name,field) uint64_t hwinfo_rs_gpu_##name(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->gpus,i):nullptr;return v?v->field:0;}
GPU_NUM(dedicated_memory,dedicated) GPU_NUM(shared_memory,shared) GPU_NUM(frequency,frequency) GPU_NUM(core_count,cores)

uint8_t hwinfo_rs_memory_present(const hwinfo_rs_snapshot*s){return s&&s->memory;} uint64_t hwinfo_rs_memory_size(const hwinfo_rs_snapshot*s){return s&&s->memory?s->memory->size:0;} uint64_t hwinfo_rs_memory_free(const hwinfo_rs_snapshot*s){return s&&s->memory?s->memory->free:0;} uint64_t hwinfo_rs_memory_available(const hwinfo_rs_snapshot*s){return s&&s->memory?s->memory->available:0;}
size_t hwinfo_rs_memory_module_count(const hwinfo_rs_snapshot*s){return s&&s->memory?s->memory->modules.size():0;} uint32_t hwinfo_rs_memory_module_id(const hwinfo_rs_snapshot*s,size_t i){auto v=s&&s->memory?at(s->memory->modules,i):nullptr;return v?v->id:UINT32_MAX;}
#define MOD_STR(name,field) const char* hwinfo_rs_memory_module_##name(const hwinfo_rs_snapshot*s,size_t i){auto v=s&&s->memory?at(s->memory->modules,i):nullptr;return v?v->field.c_str():nullptr;}
MOD_STR(vendor,vendor) MOD_STR(name,name) MOD_STR(model,model) MOD_STR(serial,serial)
uint64_t hwinfo_rs_memory_module_size(const hwinfo_rs_snapshot*s,size_t i){auto v=s&&s->memory?at(s->memory->modules,i):nullptr;return v?v->size:0;} uint64_t hwinfo_rs_memory_module_frequency(const hwinfo_rs_snapshot*s,size_t i){auto v=s&&s->memory?at(s->memory->modules,i):nullptr;return v?v->frequency:0;}

uint8_t hwinfo_rs_os_present(const hwinfo_rs_snapshot*s){return s&&s->os;} const char* hwinfo_rs_os_name(const hwinfo_rs_snapshot*s){return s&&s->os?s->os->name.c_str():nullptr;} const char* hwinfo_rs_os_version(const hwinfo_rs_snapshot*s){return s&&s->os?s->os->version.c_str():nullptr;} const char* hwinfo_rs_os_kernel(const hwinfo_rs_snapshot*s){return s&&s->os?s->os->kernel.c_str():nullptr;}
#define OS_BOOL(name,field) uint8_t hwinfo_rs_os_##name(const hwinfo_rs_snapshot*s){return s&&s->os?s->os->field:0;}
OS_BOOL(is_32_bit,bit32) OS_BOOL(is_64_bit,bit64) OS_BOOL(is_big_endian,big) OS_BOOL(is_little_endian,little)

size_t hwinfo_rs_battery_count(const hwinfo_rs_snapshot*s){return s?s->batteries.size():0;} uint32_t hwinfo_rs_battery_id(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->batteries,i):nullptr;return v?v->id:UINT32_MAX;}
#define BAT_STR(name,field) const char* hwinfo_rs_battery_##name(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->batteries,i):nullptr;return v?v->field.c_str():nullptr;}
BAT_STR(vendor,vendor) BAT_STR(model,model) BAT_STR(serial,serial) BAT_STR(technology,technology)
#define BAT_NUM(name,field,type) type hwinfo_rs_battery_##name(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->batteries,i):nullptr;return v?v->field:0;}
BAT_NUM(energy_full,full,uint32_t) BAT_NUM(energy_now,now,uint32_t) BAT_NUM(capacity,capacity,double) BAT_NUM(state,state,uint32_t)

size_t hwinfo_rs_network_count(const hwinfo_rs_snapshot*s){return s?s->networks.size():0;}
#define NET_STR(name,field) const char* hwinfo_rs_network_##name(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->networks,i):nullptr;return v?v->field.c_str():nullptr;}
NET_STR(index,index) NET_STR(description,description) NET_STR(mac,mac) NET_STR(ipv4,ipv4) NET_STR(ipv6,ipv6)
uint8_t hwinfo_rs_mainboard_present(const hwinfo_rs_snapshot*s){return s&&s->mainboard;}
#define MB_STR(name,field) const char* hwinfo_rs_mainboard_##name(const hwinfo_rs_snapshot*s){return s&&s->mainboard?s->mainboard->field.c_str():nullptr;}
MB_STR(vendor,vendor) MB_STR(name,name) MB_STR(version,version) MB_STR(serial,serial)

size_t hwinfo_rs_disk_count(const hwinfo_rs_snapshot*s){return s?s->disks.size():0;} uint32_t hwinfo_rs_disk_id(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->disks,i):nullptr;return v?v->id:UINT32_MAX;}
#define DISK_STR(name,field) const char* hwinfo_rs_disk_##name(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->disks,i):nullptr;return v?v->field.c_str():nullptr;}
DISK_STR(vendor,vendor) DISK_STR(model,model) DISK_STR(serial,serial)
uint64_t hwinfo_rs_disk_size_bytes(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->disks,i):nullptr;return v?v->size:0;} uint32_t hwinfo_rs_disk_interface(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->disks,i):nullptr;return v?v->interface_code:UINT32_MAX;} size_t hwinfo_rs_disk_mount_point_count(const hwinfo_rs_snapshot*s,size_t i){auto v=s?at(s->disks,i):nullptr;return v?v->mounts.size():0;} const char* hwinfo_rs_disk_mount_point(const hwinfo_rs_snapshot*s,size_t i,size_t j){auto v=s?at(s->disks,i):nullptr;auto x=v?at(v->mounts,j):nullptr;return str(x);}

hwinfo_rs_cpu_metrics* hwinfo_rs_cpu_metrics_collect(uint64_t ms){
#ifdef HWINFO_RS_MONITORING
  try{last_error.clear();auto v=hwinfo::monitoring::cpu::fetch(std::chrono::milliseconds(ms));return new hwinfo_rs_cpu_metrics{v.utilization,std::move(v.thread_utilization),std::move(v.thread_frequency_hz)};}catch(const std::exception&e){last_error=e.what();}catch(...){last_error="unknown native exception";}
#else
  (void)ms;last_error="monitoring feature is disabled";
#endif
  return nullptr;
}
void hwinfo_rs_cpu_metrics_free(hwinfo_rs_cpu_metrics*v){delete v;} double hwinfo_rs_cpu_metrics_utilization(const hwinfo_rs_cpu_metrics*v){return v?v->utilization:0;} size_t hwinfo_rs_cpu_metrics_thread_count(const hwinfo_rs_cpu_metrics*v){return v?v->threads.size():0;} double hwinfo_rs_cpu_metrics_thread_utilization(const hwinfo_rs_cpu_metrics*v,size_t i){auto x=v?at(v->threads,i):nullptr;return x?*x:0;} int64_t hwinfo_rs_cpu_metrics_thread_frequency(const hwinfo_rs_cpu_metrics*v,size_t i){auto x=v?at(v->frequencies,i):nullptr;return x?*x:0;}
uint8_t hwinfo_rs_memory_metrics_collect(uint64_t*f,uint64_t*a){
#ifdef HWINFO_RS_MONITORING
  try{last_error.clear();auto v=hwinfo::monitoring::ram::fetch();if(f)*f=v.free_bytes;if(a)*a=v.available_bytes;return 1;}catch(const std::exception&e){last_error=e.what();}catch(...){last_error="unknown native exception";}
#else
  (void)f;(void)a;last_error="monitoring feature is disabled";
#endif
  return 0;
}
uint8_t hwinfo_rs_disk_metrics_collect(const char*p,uint64_t*f){
#ifdef HWINFO_RS_MONITORING
  try{last_error.clear();auto v=hwinfo::monitoring::disk::fetch(p?p:"");if(f)*f=v.free_bytes;return 1;}catch(const std::exception&e){last_error=e.what();}catch(...){last_error="unknown native exception";}
#else
  (void)p;(void)f;last_error="monitoring feature is disabled";
#endif
  return 0;
}
}
