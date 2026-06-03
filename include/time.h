#pragma once
#include <stdint.h>
#include <stddef.h>

#include "sys/types.h"

#define CLOCKS_PER_SEC 1000

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
    long tm_gmtoff;
    const char *tm_zone;
};


struct timespec {
    long tv_sec;
    long tv_nsec;
};

time_t time(time_t *time);
clock_t clock(void);

time_t mktime(struct tm *time);
#define difftime(t1, t2) ((double)(t1) - (double)(t2))

struct tm *gmtime_r(time_t *time, struct tm *tm);
static inline struct tm *localtime_r(time_t *time, struct tm *tm) {
    // TODO: implement localtime_r
    return gmtime_r(time, tm);
}

static inline struct tm *localtime(time_t *time) {
    #include "stdlib.h"
    struct tm *tm = (struct tm *)calloc(1, sizeof(struct tm));
    return localtime_r(time, tm);
}

static inline struct tm *gmtime(time_t *time) {
    return localtime(time);
}


size_t strftime(char *s, size_t max, const char *format, struct tm *time);
