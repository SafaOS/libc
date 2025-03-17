pub const raw = @import("raw.zig");
const syscalls = @import("syscalls.zig");
const errno = @import("errno.zig");

pub fn zspwan(bytes: []const u8, argv: []const raw.Slice(u8), name: []const u8) errno.Error!u64 {
    const config: raw.SpawnConfig = .{ .argv = argv.ptr, .argc = argv.len, .name = .{ .ptr = name.ptr, .len = name.len }, .flags = .{ .clone_cwd = true, .clone_resources = true } };

    var pid: u64 = undefined;
    try syscalls.spawn(@ptrCast(bytes.ptr), bytes.len, &config, &pid).into_err();
    return pid;
}

pub fn zpspwan(path: []const u8, argv: []const raw.Slice(u8), name: []const u8) errno.Error!u64 {
    const config: raw.SpawnConfig = .{ .argv = argv.ptr, .argc = argv.len, .name = .{ .ptr = name.ptr, .len = name.len }, .flags = .{ .clone_cwd = true, .clone_resources = true } };
    var pid: u64 = undefined;
    try syscalls.pspawn(@ptrCast(path.ptr), path.len, &config, &pid).into_err();
    return pid;
}

pub fn wait(pid: usize) errno.Error!usize {
    var code: usize = undefined;
    try syscalls.wait(pid, &code).into_err();
    return code;
}
