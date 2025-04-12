const std = @import("std");
const builtin = std.builtin;
const builtin_info = @import("builtin");
pub const sys = @import("sys/root.zig");
pub const syscalls = sys.api.syscalls;

pub const string = @import("string.zig");
pub const stdio = @import("stdio.zig");
pub const stdlib = @import("stdlib.zig");
pub const extra = @import("extra.zig");
pub const dirent = @import("dirent.zig");
pub const math = @import("math.zig");

comptime {
    if (builtin_info.output_mode == .Lib) {
        _ = sys;
        _ = string;
        _ = stdio;
        _ = stdlib;
        _ = extra;
        _ = dirent;
        _ = math;
    }
}

// TODO: system is a stub
// TODO: kernel version 0.2.1 should have environment variables soon
pub export fn system(command: [*:0]const u8) c_int {
    _ = command;
    std.debug.panic("system() is not yet implemented", .{});
}

/// TODO: signal is a stub
pub export fn signal(sig: i32, func: *const anyopaque) *anyopaque {
    _ = sig;
    _ = func;
    std.debug.panic("signal() is not yet implemented", .{});
}

pub export fn exit(code: c_int) noreturn {
    syscalls.exit(@as(u32, @bitCast(code)));
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

const RawSlice = sys.abi.ffi.RawSlice;

fn init() void {
    stdio.stdout = stdio.init_stdout();
    stdio.stdin = stdio.init_stdin();
    stdio.stderr = stdio.init_stderr();
}

extern fn main(argc: i32, argv: [*]const [*:0]const u8) i32;
extern fn _c_start_inner(argc: usize, argv: [*]const RawSlice(u8), main: *const fn (i32, [*]const [*:0]const u8) callconv(.C) i32) noreturn;

export fn _start_inner(argc: usize, argv: [*]const RawSlice(u8)) noreturn {
    init();
    return _c_start_inner(argc, argv, main);
}

export fn _start() callconv(.naked) noreturn {
    asm volatile (
        \\ xor %rbp, %rbp
        \\ push %rbp
        \\ push %rbp
        \\ call _start_inner
    );
}
