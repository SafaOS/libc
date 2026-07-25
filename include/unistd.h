#pragma once
#include "sys/types.h"

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

int _exit(int code);
int rmdir(const char *path);

int access(const char *path, mode_t mode);
int unlink(const char *path);


off_t lseek(int fd, off_t offset, int whence);
ssize_t read(int fd, void *buf, size_t count);
ssize_t write(int fd, const void *buf, size_t count);
int close(int fd);
