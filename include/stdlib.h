#pragma once
#include <stddef.h>

int abs(int num);

int system(const char* command);
void abort() __attribute__ ((noreturn));
void exit(int code) __attribute__ ((noreturn));

void* malloc(size_t size);
void* calloc(size_t elm, size_t size);
void* realloc(void* ptr, size_t newsize);
void free(void* addr);

#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1

const char* getenv(const char *name);
int setenv(const char *name, const char *value, int overwrite);
int unsetenv(const char *name);

int atoi(const char* str);
