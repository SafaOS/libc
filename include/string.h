#pragma once
#include <stddef.h>

size_t strlen(const char* str);
size_t strnlen(const char *str, size_t maxlen);

char* strcpy(char *dest, const char *src);
char* strncpy(char *dest, const char *src, size_t count);
char* stpcpy(char *dest, const char *src);
char* strcat(char *dest, const char *src);

size_t strspn(const char* str, const char* accept);
size_t strcspn(const char *str, const char *reject);

void* memchr(const void *haystack, int needle, size_t count);
void* memset(void *dest, int c, size_t count);
void* memcpy(void *dest, const void *src, size_t count);
void* memmove(void *dest, const void *src, size_t count);

int memcmp(const void *lhs, const void *rhs, size_t count);
int strncmp(const char *lhs, const char *rhs, size_t count);
int strcmp(const char *lhs, const char *rhs);
#define strcoll strcmp

char* strrchr(const char* str, int ch);
char* strchr(const char* str, int ch);
char* strstr(const char* str, const char* substr);

char *strerror(int errnum);
double strtod(const char *str, char **str_end);
char *strpbrk(const char *s, const char *accept);
char *strdup(const char *str);
