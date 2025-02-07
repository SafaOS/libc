const syscalls = @import("syscalls.zig");
const errors = @import("errno.zig");
const seterr = errors.seterr;

pub fn zsbrk(amount: isize) errors.Error!*anyopaque {
    var ptr: usize = 0;
    const errc: u16 = @truncate(syscalls.sbrk(amount, &ptr));

    if (errc != 0) {
        const err = @errorFromInt(errc);
        const errno: errors.Error = @errorCast(err);
        return errno;
    }

    return @ptrFromInt(ptr);
}

pub export fn sbrk(amount: isize) ?*anyopaque {
    return zsbrk(amount) catch |err| {
        seterr(err);
        return null;
    };
}
