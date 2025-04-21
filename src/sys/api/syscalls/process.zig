const std = @import("std");

const raw = @import("raw.zig");
const io = @import("io.zig");

const abi = @import("../../abi/root.zig");
const ffi = abi.ffi;

const syspsawn = raw.syspspawn;

/// Spawns a new process with the given name, path, and arguments.
/// unsafe because it reuses the arguments buffer to store some data to avoid memory allocation.
pub fn unsafe_spawn(name: ?[]const u8, path: []const u8, args: [][]const u8) !raw.Pid {
    var pid: raw.Pid = undefined;

    const name_ptr = if (name) |n| n.ptr else null;
    const name_len = if (name) |n| n.len else 0;

    const argv_ptr: [*]ffi.RawSlice(u8) = @ptrCast(args.ptr);
    const argv = argv_ptr[0..args.len];

    for (args, 0..) |arg, i| {
        argv[i] = .{ .ptr = @constCast(arg.ptr), .len = arg.len };
    }

    const stdin = io.stdin().fd;
    const stdout = io.stdout().fd;
    const stderr = io.stderr().fd;

    const stdio_type = ffi.Optional(raw.Ri);

    try syspsawn(name_ptr, name_len, path.ptr, path.len, argv_ptr, args.len, 0, &pid, stdio_type.from(stdin), stdio_type.from(stdout), stdio_type.from(stderr)).into_err();
    return pid;
}

pub fn wait(pid: raw.Pid) !usize {
    var status: usize = undefined;
    try raw.syswait(pid, &status).into_err();
    return status;
}
