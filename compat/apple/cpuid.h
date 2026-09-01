#pragma once

#include <cstdint>

// lfreist/hwinfo's Intel macOS backend calls this helper but does not provide
// or include an implementation at the pinned revision.
namespace cpuid {
inline void cpuid(std::uint32_t leaf, std::uint32_t subleaf, std::uint32_t registers[4]) {
  __asm__ volatile("cpuid"
                   : "=a"(registers[0]), "=b"(registers[1]), "=c"(registers[2]), "=d"(registers[3])
                   : "a"(leaf), "c"(subleaf));
}
}  // namespace cpuid
