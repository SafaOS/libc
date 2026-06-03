#pragma once
// https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/limits.h.html



// Maximum Values


// Minimum Values

#define _POSIX_ARG_MAX                      4096
#define _POSIX_HOST_NAME_MAX                255
#define _POSIX_LINK_MAX                     8
#define _POSIX_MAX_CANON                    255
#define _POSIX_MAX_INPUT                    255
#define _POSIX_NAME_MAX                     255
#define _POSIX_OPEN_MAX                     256
#define _POSIX_PATH_MAX                     256
#define _POSIX_SYMLINK_MAX                  255


// Runtime Invariant Values (Possibly Indeterminate)

#define ATEXIT_MAX                    4096
#define OPEN_MAX                      128
#define PAGESIZE                      PAGE_SIZE
#define PTHREAD_STACK_MIN             PAGE_SIZE
#define PTHREAD_THREADS_MAX           _POSIX_THREAD_THREADS_MAX

#ifndef PAGE_SIZE
#define PAGE_SIZE 4096
#endif

// Pathname Variable Values
#define FILESIZEBITS             64
#define LINK_MAX                 _POSIX_LINK_MAX
#define MAX_CANON                _POSIX_MAX_CANON
#define MAX_INPUT                _POSIX_MAX_INPUT
#define NAME_MAX                 255
#define PATH_MAX                 4096
#define PIPE_BUF                 PAGE_SIZE




// Legacy things

#define PASS_MAX 256


// Numerical Limits
#define CHAR_MAX SCHAR_MAX
#define CHAR_MIN SCHAR_MIN

#define SCHAR_MAX __SCHAR_MAX__
#define SHRT_MAX __SHRT_MAX__
#define INT_MAX __INT_MAX__
#define LONG_MAX __LONG_MAX__
#define LLONG_MAX __LONG_LONG_MAX__
#define SSIZE_MAX __PTRDIFF_MAX__

#define SCHAR_MIN (-SCHAR_MAX - 1)
#define SHRT_MIN (-SHRT_MAX - 1)
#define INT_MIN (-INT_MAX - 1)
#define LONG_MIN (-LONG_MAX - 1)
#define LLONG_MIN (-LLONG_MAX - 1)
#define SSIZE_MIN (-SSIZE_MAX - 1)

#define USCHAR_MAX (SCHAR_MAX * 2 + 1)
#define USHRT_MAX (SHRT_MAX * 2 + 1)
#define UINT_MAX (INT_MAX * 2U + 1)
#define ULONG_MAX (LONG_MAX * 2UL + 1)
#define ULLONG_MAX (LLONG_MAX * 2ULL + 1)
