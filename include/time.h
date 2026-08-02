#pragma once
#include <stdint.h>
#include <stddef.h>
#include "stdlib.h"
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
    time_t tv_sec;
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
    struct tm *tm = (struct tm *)calloc(1, sizeof(struct tm));
    return localtime_r(time, tm);
}

static inline struct tm *gmtime(time_t *time) {
    return localtime(time);
}


size_t strftime(char *s, size_t max, const char *format, struct tm *time);



typedef int clockid_t;
#define CLOCK_REALTIME  0
#define CLOCK_MONOTONIC 1

int clock_getres(clockid_t clk_id, struct timespec *res);
int clock_gettime(clockid_t clk_id, struct timespec *tp);
int clock_settime(clockid_t clk_id, const struct timespec *tp);

int nanosleep(const struct timespec *duration,
              struct timespec *_Nullable rem);
