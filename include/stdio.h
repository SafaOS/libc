#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdarg.h>

#define EOF -1
#define BUFSIZ 4096

typedef intptr_t ssize_t;

typedef struct FILE FILE;
extern FILE* stdout;
extern FILE* stdin ;
extern FILE* stderr;

int printf(const char* fmt, ...) __attribute__((format(printf,1,2)));
int fprintf(FILE* f, const char* fmt, ...) __attribute__((format(printf,2,3)));
int vfprintf(FILE* f, const char* fmt, va_list args);
int snprintf(char* buf, size_t cap, const char* fmt, ...) __attribute__((format(printf,3,4)));
int vsnprintf(char* buf, size_t cap, const char* fmt, va_list args);
int vsprintf(char *buf, const char *fmt, va_list args);
int sprintf(char* buf, const char* fmt, ...) __attribute__((format(printf,2, 3)));

int fscanf(FILE* f, const char* fmt, ...) __attribute__((format(scanf,2,3)));
int sscanf(const char* s, const char* fmt, ...) __attribute__((format(scanf,2,3)));

FILE* fopen(const char *path, const char *mode);
FILE *freopen(const char *path, const char *mode, FILE *stream);
FILE *fdopen(int fildes, const char *mode);

int fclose(FILE *f);

size_t fwrite(const void *buffer, size_t size, size_t count, FILE *f);
size_t fread(void *buffer, size_t size, size_t count, FILE *f);

ssize_t ftell(FILE *f);

int fgetc(FILE *f);
#define getc(f) fgetc(f)
int ungetc(int c, FILE *f);
char *fgets(char *buf, size_t size, FILE *f);

#define putc(c, f) fputc(c, f)
int fputc(int c, FILE *f);

int fputs(const char *str, FILE* stream);
int puts(const char *str);

int fflush(FILE* f);

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2
int fseek(FILE *f, long offset, int whence);

int getchar();
int putchar(int c);

int remove(const char* path);
int rename(const char* old_filename, const char* new_filename);


void clearerr(FILE *f);
int ferror(FILE *f);
int feof(FILE *f);

#define FILENAME_MAX 1024

#define _IONBF 0
#define _IOFBF 1
#define _IOLBF 2

int setvbuf(FILE *stream, char *buffer, int mode, size_t size);
static int setbuf(FILE *stream, char *buffer) {
    if (buffer)
        return setvbuf(stream, buffer, _IOFBF, BUFSIZ);
    else
        return setvbuf(stream, NULL, _IONBF, 0);

}

FILE* tmpfile();
char* tmpnam(char *s);

#define L_tmpnam 128
