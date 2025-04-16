// TODO: time is a stub
pub export fn time(ptr: ?*u32) u32 {
    if (ptr) |p| p.* = 0;
    return 0;
}

// TODO: clock is a stub
pub export fn clock() u32 {
    @panic("clock(): not yet implemented");
}

pub const Time = extern struct {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    tm_wday: i32,
    tm_yday: i32,
    tm_isdst: i32,
};

export const STATIC_TIME: Time = Time{
    .tm_sec = 0,
    .tm_min = 0,
    .tm_hour = 0,
    .tm_mday = 0,
    .tm_mon = 0,
    .tm_year = 0,
    .tm_wday = 0,
    .tm_yday = 0,
    .tm_isdst = 0,
};

// TODO: localtime is a stub
pub export fn localtime(tim: *const u32) *const Time {
    _ = tim;
    return &STATIC_TIME;
}

// TODO: gmtime is a stub
pub export fn gmtime(tim: *const u32) *const Time {
    _ = tim;
    return &STATIC_TIME;
}

// TODO: mktime is a stub
pub export fn mktime(tim: *Time) u32 {
    _ = tim;
    return 0;
}

// TODO strftime is a stub
pub export fn strftime(buf: [*]u8, maxsize: usize, format: [*]const u8, tm: *const Time) usize {
    const slice = buf[0..maxsize];
    _ = format;
    _ = tm;
    @memset(slice, 0);
    @memcpy(slice, "TIME IS A STUB");
    return 0;
}
