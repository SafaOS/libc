const sys = @import("sys/root.zig");
const syscalls = sys.api.syscalls;

const string = @import("string.zig");
const stdio = @import("stdio.zig");
const stdlib = @import("stdlib.zig");
const errors = sys.abi.errors;
const seterr = errors.seterr;

const alloc = sys.api.alloc;
const raw = sys.abi.ffi;
const DirIterResource = syscalls.io.DirIterResource;
const FileResource = syscalls.io.FileResource;

pub const DIR = extern struct {
    current_index: usize = 0,
    resource: DirIterResource,

    pub fn open(path: []const u8) errors.Error!*DIR {
        const file = try FileResource.open(path);
        const diriter = try file.dirIter();

        const dir = try alloc.create(DIR);
        dir.* = .{ .resource = diriter };
        return dir;
    }

    pub fn next(self: *DIR) ?raw.DirEntry {
        defer self.current_index += 1;
        return self.resource.next();
    }

    pub fn tryClose(self: *DIR) errors.Error!void {
        try self.resource.try_close();
    }
    pub fn close(self: *DIR) void {
        defer alloc.destroy(self);
        self.resource.close();
    }
};

pub export fn opendir(path: [*:0]const c_char) ?*DIR {
    const length = string.strlen(path);
    const path_u8: [*:0]const u8 = @ptrCast(path);

    return DIR.open(path_u8[0..length]) catch |err| {
        seterr(err);
        return null;
    };
}

// FIXME: this is very unhealthy
pub export fn readdir(dir: *DIR) ?*raw.DirEntry {
    var entry = dir.next() orelse return null;
    return &entry;
}

pub export fn telldir(dir: *DIR) c_int {
    return @intCast(dir.current_index);
}

pub export fn closedir(dir: *DIR) c_int {
    dir.tryClose() catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}
