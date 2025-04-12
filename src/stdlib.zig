const std = @import("std");
const sys = @import("sys/root.zig");
const libc = @import("root.zig");
const alloc = sys.api.alloc;

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

export fn abort() noreturn {
    libc.exit(1);
}
