#pragma once
#include <stdint.h>
#include <stddef.h>

#define time_t uint32_t
#define clock_t time_t
#define CLOCKS_PER_SEC 1

struct tm {
    int tm_sec;
    int tm_min;
    int tm_hour;
    int tm_mday;
    int tm_mon;
    int tm_year;
    int tm_wday;
    int tm_yday;
    int tm_isdst;
};

time_t time(time_t *time);
clock_t clock(void);
struct tm *localtime(const time_t *time);
struct tm *gmtime(const time_t *time);

time_t mktime(struct tm *time);
#define difftime(t1, t2) ((double)(t1) - (double)(t2))
size_t strftime(char *s, size_t max, const char *format, const struct tm *tm);
