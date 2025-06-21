const std = @import("std");
const sys = @import("sys/root.zig");
const libc = @import("root.zig");

const api = sys.api;
const abi = sys.abi;
const syscalls = api.syscalls;
const env = api.env;
const alloc = api.alloc;

export fn abs(x: i32) u32 {
    return @abs(x);
}

// TODO: system is a stub
// TODO: kernel version 0.2.1 should have environment variables soon
export fn system(command_raw: [*c]const u8) c_int {
    const shell_attempt = env.get("SHELL");
    if (command_raw == null)
        return if (shell_attempt) |_| 1 else 0;

    const shell = shell_attempt orelse @panic("$SHELL not set");
    const command = std.mem.span(command_raw);

    var args = [_][]const u8{ shell, "-c", command };

    const pid = api.syscalls.process.unsafe_spawn(command, shell, &args) catch |err| {
        abi.errors.seterr(err);
        return -1;
    };

    return @intCast(api.syscalls.process.wait(pid) catch |err| {
        abi.errors.seterr(err);
        return -1;
    });
}

export fn exit(code: c_int) noreturn {
    syscalls.exit(@as(u32, @bitCast(code)));
}

export fn abort() noreturn {
    syscalls.exit(1);
}

pub const c_allocator = std.mem.Allocator{
    .ptr = undefined,
    .vtable = &c_allocator_vtable,
};

const c_allocator_vtable = std.mem.Allocator.VTable{
    .alloc = c_alloc,
    .resize = c_resize,
    .free = c_free,
    .remap = c_realloc,
};

fn c_alloc(
    _: *anyopaque,
    len: usize,
    _: std.mem.Alignment,
    _: usize,
) ?[*]u8 {
    const results = alloc.alloc(u8, len) catch return null;
    return results.ptr;
}

fn c_free(
    _: *anyopaque,
    ptr: []u8,
    _: std.mem.Alignment,
    _: usize,
) void {
    alloc.free(u8, ptr);
}

fn c_resize(
    _: *anyopaque,
    ptr: []u8,
    _: std.mem.Alignment,
    new_len: usize,
    _: usize,
) bool {
    const new_ptr = alloc.realloc(u8, ptr, new_len) catch return false;

    if (new_ptr.ptr != ptr.ptr) {
        free(ptr.ptr);
        return false;
    }

    return true;
}

fn c_realloc(
    _: *anyopaque,
    ptr: []u8,
    _: std.mem.Alignment,
    new_len: usize,
    _: usize,
) ?[*]u8 {
    const new_ptr = alloc.realloc(u8, ptr, new_len) catch return null;
    return new_ptr.ptr;
}

export fn malloc(size: usize) ?*anyopaque {
    if (size == 0) return null;
    const bytes = alloc.alloc(u8, size) catch return null;
    return @ptrCast(bytes.ptr);
}

export fn free(ptr: ?*anyopaque) void {
    if (ptr) |p| {
        alloc.destroy(p);
    }
}

export fn realloc(ptr: ?*anyopaque, size: usize) ?*anyopaque {
    if (ptr) |p| {
        if (size == 0) {
            alloc.destroy(p);
            return null;
        }

        const bytes = alloc.alloc(u8, size) catch return null;

        defer alloc.destroy(p);
        const source: [*]u8 = @ptrCast(@alignCast(p));
        @memcpy(bytes, source);

        return @ptrCast(bytes.ptr);
    } else return malloc(size);
}

export fn calloc(num: usize, size: usize) ?*anyopaque {
    if (size == 0) return null;

    const bytes_amount = num * size;
    const bytes = alloc.alloc_zeroed(u8, bytes_amount) catch return null;
    return @ptrCast(bytes.ptr);
}

export fn getenv(name: [*c]const u8) [*c]const u8 {
    if (name == null) return null;

    const result = env.get(std.mem.span(name)) orelse return null;
    const ptr: [*c]const u8 = @ptrCast(result.ptr);

    return ptr;
}

export fn setenv(name: [*c]const u8, value: [*c]const u8, overwrite: i32) c_int {
    if (name == null or value == null) return -1;

    const name_str = std.mem.span(name);
    const value_str = std.mem.span(value);

    if (overwrite != 0)
        if (env.contains(name_str)) return 0;

    env.set(name_str, value_str);
    return 0;
}

export fn unsetenv(name: [*c]const u8) c_int {
    if (name == null) return -1;
    const name_str = std.mem.span(name);
    env.remove(name_str);

    return 0;
}

export fn strtod(ptr: [*c]const u8, endptr: [*c][*c]u8) f64 {
    const str = std.mem.span(ptr);
    const str_trimmed = std.mem.trim(u8, str, &.{ ' ', '\t', '\n' });
    const result = std.fmt.parseFloat(f64, str_trimmed) catch return 0.0;

    if (endptr != null) {
        const end: [*c]u8 = @constCast(ptr + str_trimmed.len);
        endptr.* = @ptrCast(end);
    }

    return result;
}

export fn atoi(c_str: [*c]const u8) c_int {
    const str = std.mem.span(c_str);
    return std.fmt.parseInt(c_int, str, 10) catch 0;
}
