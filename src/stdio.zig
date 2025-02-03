const io = @import("sys/io.zig");
const string = @import("string.zig");
const extra = @import("extra.zig");
const panic = @import("root.zig").panic;
const stdlib = @import("stdlib.zig");
const syscalls = @import("sys/syscalls.zig");
const errors = @import("sys/errno.zig");
const geterr = errors.geterr;
const seterr = errors.seterr;
const std = @import("std");
const allocator = stdlib.c_allocator;

const EOF: u8 = 255;

const VaList = @import("std").builtin.VaList;

pub const ModeFlags = packed struct {
    read: bool = false,
    write: bool = false,
    append: bool = false,
    extended: bool = false,
    access_flag: bool = false,

    _padding: u3 = 0,
    pub fn from_cstr(cstr: [*:0]const c_char) ?@This() {
        var bytes: [*:0]const u8 = @ptrCast(cstr);
        var self: ModeFlags = .{};

        while (bytes[0] != 0) : (bytes += 1) {
            const byte = bytes[0];
            switch (byte) {
                'w' => self.write = true,
                'a' => self.append = true,
                'r' => self.read = true,
                '+' => self.extended = true,
                'x' => self.access_flag = true,
                else => return null,
            }
        }

        return self;
    }
};

pub const FileWriter = std.io.Writer(*FILE, errors.Error, File.write);
pub const FileReader = std.io.Reader(*FILE, errors.Error, File.read);

pub const FILE = extern struct {
    const Self = @This();
    fd: usize,
    mode: ModeFlags,
    read_offset: isize = 0,
    write_offset: isize = 0,

    pub fn open(filename: []const u8, mode: ModeFlags) errors.Error!*Self {
        const fd = io.zopen(filename) catch |err| blk: {
            switch (err) {
                error.NoSuchAFileOrDirectory => if (mode.write or mode.append) {
                    try io.zcreate(filename);
                    break :blk try io.zopen(filename);
                } else return err,
                else => return err,
            }
        };

        if (mode.write) {
            if (mode.access_flag) {
                return error.AlreadyExists;
            }
            io.ztruncate(@bitCast(fd), 0) catch {};
        }

        const file = allocator.create(FILE) catch unreachable;
        file.* = .{ .fd = fd, .mode = mode };
        return file;
    }

    pub fn closeChecked(file: *FILE) errors.Error!void {
        defer stdlib.free(file);
        try io.zclose(file.fd);
    }

    pub fn close(file: *Self) void {
        file.closeChecked() catch unreachable;
    }

    pub fn flush(file: *Self) errors.Error!void {
        try io.zsync(@bitCast(file.fd));
    }

    pub fn writer(self: *FILE) FileWriter {
        return .{ .context = self };
    }

    pub fn reader(self: *FILE) FileReader {
        return .{ .context = self };
    }

    fn check_read(self: *const Self) errors.Error!void {
        if (!self.mode.read) return error.MissingPermissions;
    }

    fn check_write(self: *const Self) errors.Error!void {
        if (!self.mode.write) return error.MissingPermissions;
    }

    pub fn read(self: *Self, buf: []u8) errors.Error!usize {
        try self.check_read();
        const amount = io.zread(self.fd, self.read_offset, buf) catch |err| {
            switch (err) {
                error.InvaildOffset => return 0,
                else => return err,
            }
        };
        self.read_offset += @intCast(amount);
        return amount;
    }

    pub fn readByte(self: *Self) errors.Error!u8 {
        var buffer: [1]u8 = undefined;
        const amount = try self.read(&buffer);
        if (amount == 0) return EOF;
        return buffer[0];
    }

    pub fn write(self: *Self, buf: []const u8) errors.Error!usize {
        try self.check_write();
        const amount = io.zwrite(self.fd, self.write_offset, buf) catch |err| {
            switch (err) {
                error.InvaildOffset => return 0,
                else => return err,
            }
        };
        self.write_offset += @intCast(amount);

        if (buf[buf.len - 1] == '\n') try self.flush();
        return amount;
    }

    pub fn writeByte(self: *Self, c: u8) errors.Error!void {
        return self.write(&[1]u8{c});
    }
};

pub const File = FILE;
pub export var stdin: FILE = .{ .fd = 0, .mode = .{ .read = true } };
pub export var stdout: FILE = .{ .fd = 1, .mode = .{ .write = true } };

export fn fopen(filename: [*:0]const c_char, mode: [*:0]const c_char) ?*FILE {
    const path: [*:0]const u8 = @ptrCast(filename);
    const len = string.strlen(filename);
    const modeflags = ModeFlags.from_cstr(mode) orelse {
        seterr(error.InvaildStr);
        return null;
    };

    return FILE.open(path[0..len], modeflags) catch |err| {
        seterr(err);
        return null;
    };
}

export fn fclose(file: *FILE) c_int {
    FILE.closeChecked(file) catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}

fn zfgetc(stream: *FILE) errors.Error!u8 {
    return stream.readByte();
}

export fn fgetc(stream: *FILE) c_int {
    const c = zfgetc(stream) catch |err| {
        seterr(err);
        return -1;
    };

    return @intCast(c);
}

export fn getc(stream: *FILE) c_int {
    return fgetc(stream);
}

export fn getchar() c_int {
    return fgetc(&stdin);
}

fn zfgetline(file: *FILE) errors.Error![]u8 {
    const opt = try file.reader().readUntilDelimiterOrEofAlloc(allocator, '\n', std.math.maxInt(usize));
    var buffer = try allocator.realloc(opt.?, opt.?.len + 2);
    buffer[buffer.len - 2] = '\n';
    buffer[buffer.len - 1] = 0;
    return buffer;
}

pub fn zgetline() !?[]u8 {
    return stdin.reader().readUntilDelimiterOrEofAlloc(allocator, '\n', std.math.maxInt(usize));
}

export fn fgetline(file: *FILE, len: *usize) ?[*]c_char {
    const slice = zfgetline(file) catch |err| {
        seterr(err);
        return null;
    };
    len.* = slice.len;
    return @ptrCast(slice.ptr);
}

fn wc(c: u8) isize {
    return io.write(1, &c, 1);
}

pub fn zprintf(comptime fmt: []const u8, args: anytype) void {
    const writer = stdout.writer();
    writer.print(fmt, args) catch {};
}
