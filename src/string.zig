const errors = @import("sys/errno.zig");
const Error = errors.Error;
const std = @import("std");

pub export fn strlen(cstr: [*:0]const c_char) usize {
    var i: usize = 0;
    while (cstr[i] != 0)
        i += 1;
    return i;
}
pub export fn strerror(errnum: u32) [*:0]const c_char {
    if (errnum >= @intFromError(Error.Last)) {
        return @ptrCast("UNKNOWN");
    }

    if (errnum == 0) {
        return @ptrCast("NO ERROR");
    }

    const errnum_short: u16 = @truncate(errnum);
    const err: Error = @errorCast(@errorFromInt(errnum_short));
    return @ptrCast(@errorName(err));
}

pub export fn strerrorlen_s(errnum: u32) usize {
    return strlen(strerror(errnum));
}

pub export fn memset(str: [*]void, c: c_int, n: usize) [*]void {
    const char_str: [*]u8 = @ptrCast(str);
    const char: u8 = @intCast(c);

    for (0..n) |i| {
        char_str[i] = char;
    }

    return @ptrCast(char_str);
}

pub export fn memmove(dest_raw: [*]u8, src_raw: [*]const u8, n: usize) [*]u8 {
    const dest = dest_raw[0..n];
    const src = src_raw[0..n];

    if (@intFromPtr(dest.ptr) <= @intFromPtr(src.ptr)) {
        std.mem.copyForwards(u8, dest, src);
    } else {
        std.mem.copyBackwards(u8, dest, src);
    }

    return dest.ptr;
}

pub export fn memcmp(s1: [*]const void, s2: [*]const void, n: usize) c_int {
    const byte_s1: [*]const u8 = @ptrCast(s1);
    const byte_s2: [*]const u8 = @ptrCast(s2);

    var i: usize = 0;
    while (i < n) : (i += 1) {
        if (byte_s1[i] < byte_s2[i]) {
            return -1;
        } else if (byte_s1[i] > byte_s2[i]) {
            return 1;
        }
    }
    return 0;
}

pub fn zmemcpy(comptime T: type, dest: []T, src: []const T) void {
    _ = @memcpy(dest, src);
}
