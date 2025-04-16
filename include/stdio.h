#pragma once
#include <stdint.h>
#include <stddef.h>

#define EOF -1
#define BUFSIZ 4096

typedef intptr_t ssize_t;

typedef struct FILE FILE;
extern FILE* stdout;
extern FILE* stdin ;
extern FILE* stderr;

ssize_t printf(const char* fmt, ...) __attribute__((format(printf,1,2)));
ssize_t fprintf(FILE* f, const char* fmt, ...) __attribute__((format(printf,2,3)));
ssize_t snprintf(char* buf, size_t cap, const char* fmt, ...) __attribute__((format(printf,3,4)));
ssize_t sprintf(char* buf, const char* fmt, ...) __attribute__((format(printf,2, 3)));

FILE* fopen(const char *path, const char *mode);
FILE *freopen(const char *path, const char *mode, FILE *stream);

int fclose(FILE *f);

size_t fwrite(const void *buffer, size_t size, size_t count, FILE *f);
size_t fread(void *buffer, size_t size, size_t count, FILE *f);

ssize_t ftell(FILE *f);

int fgetc(FILE *f);
#define getc(f) fgetc(f)
int ungetc(int c, FILE *f);
char *fgets(char *buf, size_t size, FILE *f);
int fputc(int c, FILE *f);

int fputs(const char *str, FILE* stream);

static inline int puts(const char *str) {
    return fputs(str, stdout);
}

int fflush(FILE* f);

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2
int fseek(FILE *f, long offset, int whence);

int getchar();

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

FILE* tmpfile();
char* tmpnam(char *s);

#define L_tmpnam 128
