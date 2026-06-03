#pragma once
#include <stddef.h>

int abs(int num);

int system(const char* command);
void exit(int code) __attribute__ ((noreturn));
void atexit(void (*func(void)));

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
double atof(const char* str);

void srand(unsigned int seed);
int rand(void);
