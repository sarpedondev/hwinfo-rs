#pragma once

// lfreist/hwinfo uses the Windows SDK casing. MinGW-w64 installs this header
// in lowercase, which matters when cross-compiling on a case-sensitive host.
#include <wbemidl.h>
