#pragma once
struct timeval {
    long tv_sec;
    long tv_usec;
};

struct timezone {
    int tz_minuteswest;
    int tz_dsttime;
};

int gettimeofday(struct timeval *tv, struct timezone *tz);
static int utimes(const char *path, const struct timeval times[2]) {
    return 0;
}
