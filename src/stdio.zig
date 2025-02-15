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

pub fn CWriter(comptime T: type, comptime func: fn (T, []const u8) errors.Error!usize) type {
    return std.io.Writer(T, errors.Error, func);
}

pub fn CReader(comptime T: type, comptime func: fn (T, []u8) errors.Error!usize) type {
    return std.io.Reader(T, errors.Error, func);
}

pub const FileWriter = CWriter(*FILE, File.write);
pub const FileReader = CReader(*FILE, File.read);
pub const SeekWhence = enum(usize) {
    Set = 0,
    Current = 1,
    End = 2,
};

pub const FILE = extern struct {
    const Self = @This();
    fd: usize,
    mode: ModeFlags,
    seek_at: isize = 0,
    write_at: usize = 0,
    // TODO: make this dynamic based on buffering options
    write_buffer: [1024]u8 = undefined,
    read_at: usize = 0,
    read_len: usize = 0,
    read_buffer: [1024]u8 = undefined,

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
        file.flush() catch |err| {
            switch (err) {
                error.OperationNotSupported => return,
                else => return err,
            }
        };
        try io.zclose(file.fd);
    }

    pub fn close(file: *Self) void {
        file.closeChecked() catch unreachable;
    }

    pub fn flush(self: *Self) errors.Error!void {
        try self.flush_write();
        try self.flush_read();
        try self.sync();
    }

    fn flush_write(self: *Self) errors.Error!void {
        if (!self.mode.write) return;
        if (self.write_at == 0) return;
        const amount = io.zwrite(self.fd, self.seek_at, self.write_buffer[0..self.write_at]) catch |err| blk: {
            switch (err) {
                error.InvaildOffset, error.OperationNotSupported => break :blk 0,
                else => return err,
            }
        };
        self.seek_at += @intCast(amount);
        self.write_at = 0;
    }

    fn flush_read(self: *Self) errors.Error!void {
        if (!self.mode.read) return;
        if (self.read_at == self.read_buffer.len) return;
        const amount = io.zread(self.fd, self.seek_at, self.read_buffer[self.read_at..]) catch |err| blk: {
            switch (err) {
                error.InvaildOffset, error.OperationNotSupported => break :blk 0,
                else => return err,
            }
        };

        self.seek_at += @intCast(amount);
        self.read_len = self.read_at + amount;
    }

    pub fn sync(file: *Self) errors.Error!void {
        try io.zsync(file.fd);
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

    // an optimized version of read for reading a single byte
    fn readBytePtr(self: *Self, ptr: *u8) errors.Error!usize {
        if (self.read_len - self.read_at < 1) try self.flush();
        if (self.read_len == 0) return 0;

        ptr.* = self.read_buffer[self.read_at];
        self.read_at += 1;
        // request to restart reading if we don't have enough in the buffer
        if (self.read_at >= self.read_buffer.len) {
            self.read_at = 0;
            try self.flush();
        }
        return 1;
    }

    pub fn read(self: *Self, buf: []u8) errors.Error!usize {
        try self.check_read();
        if (buf.len == 0) return 0;
        if (buf.len == 1) return self.readBytePtr(&buf[0]);

        // request more data if we don't have enough in the buffer
        if (self.read_len - self.read_at < buf.len) try self.flush();
        const read_buffer = self.read_buffer[self.read_at..self.read_len];
        const amount = @min(buf.len, read_buffer.len);
        @memcpy(buf[0..amount], read_buffer[0..amount]);

        self.read_at += amount;
        // request to restart reading if we don't have enough in the buffer
        if (self.read_at >= self.read_buffer.len) {
            self.read_at = 0;
            try self.flush();
        }
        return amount;
    }

    pub fn readByte(self: *Self) errors.Error!u8 {
        try self.check_read();
        var c: u8 = undefined;

        const amount = try self.readBytePtr(&c);
        if (amount == 0) return EOF;

        return c;
    }

    pub fn write(self: *Self, buf: []const u8) errors.Error!usize {
        try self.check_write();
        if (buf.len == 0) return 0;

        var write_buffer = self.write_buffer[self.write_at..];
        const amount = @min(buf.len, write_buffer.len);
        @memcpy(write_buffer[0..amount], buf[0..amount]);

        self.write_at += amount;
        // TODO: change this when we have different buffering options
        if (buf[buf.len - 1] == '\n' or self.write_at >= self.write_buffer.len) try self.flush();
        return amount;
    }

    pub fn writeByte(self: *Self, c: u8) errors.Error!void {
        // FIXME: EOF
        _ = try self.write(&[1]u8{c});
    }

    pub fn seek(self: *Self, offset: isize, whence: SeekWhence) errors.Error!void {
        switch (whence) {
            .Set => self.seek_at = offset,
            .Current => self.seek_at += offset,
            .End => self.seek_at = -1 - offset,
        }
    }

    /// helper to write a formatted string to a file in a C-style format
    fn traverseFmt(self: std.io.AnyWriter, fmt: [*:0]const u8) !?[*:0]const u8 {
        var current = fmt;
        var len: usize = 0;
        while (current[0] != '%' and current[0] != 0) {
            current += 1;
            len += 1;
        }

        _ = try self.write(fmt[0..len]);
        if (current[0] == 0) return null;
        return current;
    }

    /// writes a formatted string to the file in a C-style format
    pub fn writeVarFmt(self: std.io.AnyWriter, fmt: [*:0]const u8, args: *VaList) !void {
        var current = fmt;

        while (current[0] != 0) : (current += 1) {
            current = try Self.traverseFmt(self, current) orelse return;
            current += 1;
            switch (current[0]) {
                'd' => {
                    const i = @cVaArg(args, i32);
                    try self.print("{}", .{i});
                },

                'u' => {
                    const i = @cVaArg(args, u32);
                    try self.print("{}", .{i});
                },

                'z' => {
                    if (current[1] == 'u') {
                        current += 1;
                        const i = @cVaArg(args, usize);
                        try self.print("{}", .{i});
                    } else {
                        const i = @cVaArg(args, isize);
                        try self.print("{}", .{i});
                    }
                },

                'l' => {
                    if (current[1] == 'u') {
                        current += 1;
                        const i = @cVaArg(args, u64);
                        try self.print("{}", .{i});
                    } else {
                        const i = @cVaArg(args, i64);
                        try self.print("{}", .{i});
                    }
                },

                'p', 'x' => {
                    const i = @cVaArg(args, usize);
                    try self.print("{x}", .{i});
                },

                's' => {
                    const str = @cVaArg(args, [*:0]const u8);
                    try self.print("{s}", .{str});
                },

                '.' => {
                    current += 1;
                    switch (current[0]) {
                        '*' => {
                            current += 1;
                            const length = @cVaArg(args, usize);
                            switch (current[0]) {
                                's' => {
                                    const str = @cVaArg(args, [*]const u8);
                                    try self.print("{s}", .{str[0..length]});
                                },
                                else => {},
                            }
                        },
                        else => {},
                    }
                },

                else => continue,
            }
        }
    }
};

pub const File = FILE;
pub var stdin_desc: FILE = .{ .fd = 0, .mode = .{ .read = true } };
pub export var stdin: *FILE = &stdin_desc;

pub var stdout_desc: FILE = .{ .fd = 1, .mode = .{ .write = true } };
pub export var stdout: *FILE = &stdout_desc;

pub var stderr_desc: FILE = .{ .fd = 2, .mode = .{ .write = true } };
pub export var stderr: *FILE = &stderr_desc;

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

export fn fread(ptr: [*]u8, size: usize, count: usize, stream: *FILE) usize {
    const amount = stream.read(ptr[0 .. size * count]) catch |err| {
        seterr(err);
        return 0;
    };
    return amount / size;
}

export fn fwrite(ptr: [*]const u8, size: usize, count: usize, stream: *FILE) usize {
    const amount = stream.write(ptr[0 .. size * count]) catch |err| {
        seterr(err);
        return 0;
    };
    return amount / size;
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
    return fgetc(stdin);
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

export fn fprintf(stream: *FILE, fmt: [*:0]const u8, ...) c_int {
    var args = @cVaStart();
    var writer = stream.writer();
    File.writeVarFmt(writer.any(), fmt, &args) catch |err| {
        seterr(@errorCast(err));
        return -1;
    };
    return 0;
}

export fn snprintf(str: [*:0]u8, size: usize, fmt: [*:0]const u8, ...) c_int {
    var args = @cVaStart();
    var stream = std.io.fixedBufferStream(str[0..size]);
    var writer = stream.writer();
    File.writeVarFmt(writer.any(), fmt, &args) catch |err| {
        seterr(@errorCast(err));
        return -1;
    };
    return 0;
}

export fn printf(fmt: [*:0]const u8, ...) c_int {
    var args = @cVaStart();
    var writer = stdout.writer();
    File.writeVarFmt(writer.any(), fmt, &args) catch |err| {
        seterr(@errorCast(err));
        return -1;
    };
    return 0;
}

export fn fflush(stream: *FILE) c_int {
    FILE.flush(stream) catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}

export fn fputc(c: u8, stream: *FILE) c_int {
    FILE.writeByte(stream, c) catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}

export fn fseek(stream: *FILE, offset: isize, whence: u8) c_int {
    if (whence > @intFromEnum(SeekWhence.End)) {
        seterr(error.Generic);
        return -1;
    }
    const seek_whence: SeekWhence = @enumFromInt(whence);

    FILE.seek(stream, offset, seek_whence) catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}

export fn ftell(stream: *FILE) usize {
    if (stream.seek_at >= 0) {
        return @bitCast(stream.seek_at);
    } else {
        var size: usize = undefined;
        _ = syscalls.fsize(stream.fd, &size);
        const pos_from_end: usize = @bitCast(-stream.seek_at - 1);
        return size - pos_from_end;
    }
}
