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

pub export fn memcpy(dest: [*]void, src: [*]const void, size: usize) [*]void {
    const byte_dest: [*]u8 = @ptrCast(dest);
    const byte_src: [*]const u8 = @ptrCast(src);
    for (0..size) |i| {
        byte_dest[i] = byte_src[i];
    }

    return dest;
}

pub export fn memmove(dest: [*]void, src: [*]const void, n: usize) [*]void {
    var size = n;
    var byte_dest: [*]u8 = @ptrCast(dest);
    var byte_src: [*]const u8 = @ptrCast(src);

    if (@intFromPtr(byte_dest) > @intFromPtr(byte_src) and @intFromPtr(byte_src + size) > @intFromPtr(byte_dest)) {
        byte_src += size;
        byte_dest += size;

        while (size != 0) {
            size -= 1;
            byte_dest[size] = byte_src[size];
        }
    } else {
        for (0..n) |i| {
            byte_dest[i] = byte_src[i];
        }
    }

    return dest;
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
    _ = memcpy(@ptrCast(dest.ptr), @ptrCast(src.ptr), @sizeOf(T) * src.len);
}
