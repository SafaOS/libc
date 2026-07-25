#pragma once
#include "sys/types.h"
int  open(const char *, int, ...);
int  creat(const char *, mode_t);

#define O_APPEND (1 << 0)
#define O_CREAT  (1 << 1)
#define O_EXCL   (1 << 2)
#define O_TRUNC  (1 << 3)
#define O_WRONLY (1 << 4)
#define O_RDONLY (1 << 5)
#define O_RDWR   (1 << 6)
