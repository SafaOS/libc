const sys = @import("sys/root.zig");
const errors = sys.abi.errors;
const Error = errors.Error;
const SysError = errors.SysError;
const std = @import("std");

pub export fn strlen(cstr: [*:0]const c_char) usize {
    return std.mem.len(cstr);
}

pub export fn strnlen(cstr: [*:0]const c_char, maxlen: usize) usize {
    var i: usize = 0;
    while (i < maxlen and cstr[i] != 0)
        i += 1;
    return i;
}

pub export fn strcpy(dest: [*]u8, src: [*:0]const u8) [*]u8 {
    const len = strlen(@ptrCast(src));
    const dest_slice = dest[0 .. len + 1];
    @memcpy(dest_slice, src);
    return dest;
}

pub export fn strncpy(dest: [*]u8, src: [*:0]const u8, n: usize) [*]u8 {
    const len = strnlen(@ptrCast(src), n);
    const dest_slice = dest[0 .. len + 1];
    @memcpy(dest_slice, src);
    return dest;
}

pub export fn stpcpy(dest: [*]u8, src: [*:0]const u8) [*]u8 {
    const len = strlen(@ptrCast(src));
    const dest_slice = dest[0 .. len + 1];
    @memcpy(dest_slice, src);
    return dest + len;
}

pub export fn strspn(s_ptr: [*:0]const u8, accept_ptr: [*:0]const u8) usize {
    const accept_len = strlen(@ptrCast(accept_ptr));
    const s_len = strlen(@ptrCast(s_ptr));

    const accept = accept_ptr[0..accept_len];
    const s = s_ptr[0..s_len];

    var i: usize = 0;

    for (s) |c|
        if (std.mem.indexOfScalar(u8, accept, c)) |_| {
            i += 1;
        } else break;

    return i;
}

pub export fn strcspn(s_ptr: [*:0]const u8, reject_ptr: [*:0]const u8) usize {
    const reject_len = strlen(@ptrCast(reject_ptr));
    const s_len = strlen(@ptrCast(s_ptr));

    const reject = reject_ptr[0..reject_len];
    const s = s_ptr[0..s_len];

    var i: usize = 0;

    for (s) |c|
        if (std.mem.indexOfScalar(u8, reject, c) == null) {
            i += 1;
        } else break;

    return i;
}

pub export fn memchr(p: [*]const u8, c: c_int, n: usize) ?[*]const u8 {
    const char: u8 = @intCast(c);
    const char_p = p[0..n];

    const index = std.mem.indexOfScalar(u8, char_p, char) orelse return null;
    return p + index;
}

pub export fn strchr(s: [*:0]const u8, c: c_int) ?[*:0]const u8 {
    const char: u8 = @intCast(c);
    var char_s = s;

    while (char_s[0] != 0) {
        if (char_s[0] == char) {
            return char_s;
        }
        char_s += 1;
    }
    return null;
}

pub export fn strrchr(s: [*:0]const u8, c: c_int) ?[*:0]const u8 {
    const char: u8 = @intCast(c);
    const len = strlen(@ptrCast(s));
    const char_s = s[0..len];
    const index = std.mem.lastIndexOfScalar(u8, char_s, char) orelse return null;
    return s + index;
}

pub export fn strstr(haystack_ptr: [*:0]const u8, needle_ptr: [*:0]const u8) ?[*:0]const u8 {
    const needle_len = strlen(@ptrCast(needle_ptr));
    const haystack_len = strlen(@ptrCast(haystack_ptr));

    const haystack = haystack_ptr[0..haystack_len];
    const needle = needle_ptr[0..needle_len];
    const index = std.mem.indexOf(u8, haystack, needle) orelse return null;

    return haystack_ptr + index;
}

pub export fn strcat(dest: [*:0]u8, src: [*:0]const u8) [*]u8 {
    const len = strlen(@ptrCast(dest));
    const src_len = strlen(@ptrCast(src));
    @memcpy(dest[len .. len + src_len + 1], src);
    return dest;
}

pub export fn strcmp(s1: [*:0]const u8, s2: [*:0]const u8) c_int {
    return switch (std.mem.orderZ(u8, s1, s2)) {
        .eq => 0,
        .gt => 1,
        .lt => -1,
    };
}

pub export fn strncmp(s1: [*:0]const u8, s2: [*:0]const u8, n: usize) c_int {
    const len1 = strnlen(@ptrCast(s1), n);
    const len2 = strnlen(@ptrCast(s2), n);
    return switch (std.mem.order(u8, s1[0..len1], s2[0..len2])) {
        .eq => 0,
        .gt => 1,
        .lt => -1,
    };
}

pub export fn strerror(errnum: u32) [*:0]const c_char {
    const no_error: [*:0]const c_char = @ptrCast("NO ERROR");
    const unknown: [*:0]const c_char = @ptrCast("UNKNOWN");

    if (errnum == 0) {
        return no_error;
    }

    if (errnum >= std.math.maxInt(u16)) {
        return unknown;
    }
    const errnum_short: u16 = @truncate(errnum);
    const syserror = SysError.from_u16(errnum_short) orelse return unknown;
    return @ptrCast(@tagName(syserror));
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

pub export fn memcmp(s1: [*]const u8, s2: [*]const u8, n: usize) c_int {
    const byte_s1: []const u8 = s1[0..n];
    const byte_s2: []const u8 = s2[0..n];
    return switch (std.mem.order(u8, byte_s1, byte_s2)) {
        .eq => 0,
        .gt => 1,
        .lt => -1,
    };
}

pub fn zmemcpy(comptime T: type, dest: []T, src: []const T) void {
    _ = @memcpy(dest, src);
}

pub export fn strpbrk(dest: [*c]const u8, accept: [*c]const u8) [*c]u8 {
    const result = std.mem.indexOfAny(u8, std.mem.span(dest), std.mem.span(accept)) orelse return null;
    return @constCast(dest + result);
}
