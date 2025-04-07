const abi = @import("../abi/root.zig");
const Optional = abi.ffi.Optional;
const RawSlice = abi.ffi.RawSlice;

extern fn syscreate(object_size: usize) Optional(RawSlice(u8));
extern fn sysdestroy(object: *const anyopaque) void;

pub const AllocError = error{
    OutOfMemory,
};

pub fn create(comptime T: type) AllocError!*T {
    const allocated = try alloc(T, 1);
    return &allocated[0];
}

pub fn alloc(comptime T: type, len: usize) AllocError![]T {
    const object_size = @sizeOf(T) * len;
    const bytes_raw = syscreate(object_size).into() orelse return AllocError.OutOfMemory;
    const bytes = bytes_raw.into();
    const slice_ptr: [*]T = @ptrCast(@alignCast(bytes.ptr));

    return slice_ptr[0..len];
}

pub fn realloc(comptime T: type, slice: []T, new_len: usize) AllocError![]T {
    defer free(T, slice);

    const new_slice = try alloc(T, new_len);
    @memcpy(new_slice, slice);
    return new_slice;
}

pub fn destroy(object: *anyopaque) void {
    sysdestroy(object);
}

pub fn free(comptime T: type, slice: []T) void {
    destroy(@ptrCast(slice.ptr));
}
