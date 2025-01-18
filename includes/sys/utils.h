#ifndef __nlibc__SRC_SYS_UTILS_
#define __nlibc__SRC_SYS_UTILS_

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <sys/types.h>

#include "raw.h"
SysInfo *sysinfo();
ssize_t pcollect(ProcessInfo *arg0, size_t arg1);

#endif