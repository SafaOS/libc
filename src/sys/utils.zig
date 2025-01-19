pub const raw = @import("raw.zig");
const syscalls = @import("syscalls.zig");
const errno = @import("errno.zig");

pub fn zspwan(bytes: []const u8, argv: []const raw.Slice(u8), name: []const u8) errno.Error!u64 {
    const config: raw.SpawnConfig = .{ .argv = argv.ptr, .argc = argv.len, .name = .{ .ptr = name.ptr, .len = name.len }, .flags = .{ .clone_cwd = true, .clone_resources = true } };

    var pid: u64 = undefined;
    const err = syscalls.spawn(@ptrCast(bytes.ptr), bytes.len, &config, &pid);
    if (err != 0) {
        const res: u32 = @truncate(err);
        errno.errno = res;
        return errno.geterr();
    }

    return pid;
}

pub fn zpspwan(path: []const u8, argv: []const raw.Slice(u8), name: []const u8) errno.Error!u64 {
    const config: raw.SpawnConfig = .{ .argv = argv.ptr, .argc = argv.len, .name = .{ .ptr = name.ptr, .len = name.len }, .flags = .{ .clone_cwd = true, .clone_resources = true } };

    var pid: u64 = undefined;
    const err = syscalls.pspawn(@ptrCast(path.ptr), path.len, &config, &pid);
    if (err != 0) {
        const res: u32 = @truncate(err);
        errno.errno = res;
        return errno.geterr();
    }

    return pid;
}
