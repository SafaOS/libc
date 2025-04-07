const std = @import("std");
const builtin = std.builtin;
const builtin_info = @import("builtin");
pub const sys = @import("sys/root.zig");
pub const syscalls = sys.api.syscalls;

pub const ctype = @import("ctype.zig");
pub const string = @import("string.zig");
pub const stdio = @import("stdio.zig");
pub const stdlib = @import("stdlib.zig");
pub const extra = @import("extra.zig");
pub const dirent = @import("dirent.zig");

comptime {
    // TODO: figure out a method to not export unused stuff
    if (builtin_info.output_mode == .Lib) {
        _ = sys;
        _ = ctype;
        _ = string;
        _ = stdio;
        _ = stdlib;
        _ = extra;
        _ = dirent;
    }
}

pub export fn exit(code: usize) noreturn {
    syscalls.exit(code);
}

pub fn panic(msg: []const u8, error_return_trace: ?*builtin.StackTrace, return_addr: ?usize) noreturn {
    const at = return_addr orelse @returnAddress();

    stdio.zprintf("\x1B[38;2;200;0;0mlibc panic: {s} at 0x{x} <??>\n", .{ msg, at });
    stdio.zprintf("trace:\n", .{});

    if (error_return_trace) |trace| {
        const addresses = trace.instruction_addresses;

        for (addresses) |address| {
            stdio.zprintf("  <0x{x}>\n", .{address});
        }
    } else {
        var rbp: ?[*]usize = @ptrFromInt(@frameAddress());
        while (rbp != null) : (rbp = @ptrFromInt(rbp.?[0])) {
            stdio.zprintf("  0x{x} <??>\n", .{rbp.?[1]});
        }
    }
    stdio.zprintf("\x1B[0m", .{});

    exit(1);
}

export fn __assert_fail(expr: [*:0]const u8, file: [*:0]const u8, line: u32, function: ?[*:0]const u8) noreturn {
    stdio.zprintf("assertion failed: {s} at {s}:{d}", .{ expr, file, line });
    if (function != null) {
        stdio.zprintf(" function {s}", .{function.?});
    }
    stdio.zprintf("\n", .{});
    @panic("assetion failed");
}
