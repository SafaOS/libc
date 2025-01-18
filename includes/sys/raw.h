#ifndef __nlibc__SRC_SYS_RAW_
#define __nlibc__SRC_SYS_RAW_

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <sys/types.h>

typedef struct DirEntry {
  uint8_t kind;
  size_t size;
  size_t name_length;
  uint8_t name[128];
} DirEntry;

typedef struct SpawnConfig {
  struct {
    const uint8_t *ptr;
    size_t len;
  } name;
  const struct {
    const uint8_t *ptr;
    size_t len;
  } *argv;
  size_t argc;
  uint8_t flags;
} SpawnConfig;

typedef struct SysInfo {
  size_t total_mem;
  size_t used_mem;
  size_t processes_count;
} SysInfo;

typedef enum ProcessStatus: uint8_t {
  Waiting, 
  Running, 
  Zombie, 
} ProcessStatus;

typedef struct ProcessInfo {
  uint64_t ppid;
  uint64_t pid;
  uint8_t name[64];
  ProcessStatus status;
  size_t resource_count;
  size_t exit_code;
  size_t exit_addr;
  size_t exit_stack_addr;
  uint64_t killed_by;
  size_t data_start;
  size_t data_break;
} ProcessInfo;

typedef struct OsStr {
  size_t len;
  uint8_t data_off[1];
} OsStr;


#endif