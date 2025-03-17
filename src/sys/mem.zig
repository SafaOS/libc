const syscalls = @import("syscalls.zig");
const errors = @import("errno.zig");
const seterr = errors.seterr;

pub fn zsbrk(amount: isize) errors.Error!*anyopaque {
    var ptr: usize = 0;
    try syscalls.sbrk(amount, &ptr).into_err();
    return @ptrFromInt(ptr);
}

pub export fn sbrk(amount: isize) ?*anyopaque {
    return zsbrk(amount) catch |err| {
        seterr(err);
        return null;
    };
}
