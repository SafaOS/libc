//! a higher level wrapper around sys/api/syscalls/io.zig
const std = @import("std");
const syscalls = @import("syscalls/root.zig");
const abi = @import("../abi/root.zig");

const io = syscalls.io;
const errors = abi.errors;
const FileResource = io.FileResource;

pub const ModeFlags = packed struct {
    read: bool = false,
    write: bool = false,
    append: bool = false,
    extended: bool = false,
    access_flag: bool = false,
    binary: bool = false,

    _padding: u2 = 0,
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
                'b' => self.binary = true,
                else => return null,
            }
        }

        return self;
    }
};

pub const File = struct {
    resource: io.FileResource,
    mode: ModeFlags,
    offset: isize = 0,

    const Self = @This();

    pub fn open(filename: []const u8, mode: ModeFlags) errors.Error!Self {
        const fd = FileResource.open(filename) catch |err| blk: {
            switch (err) {
                error.NoSuchAFileOrDirectory => if (mode.write or mode.append) {
                    try syscalls.io.create(filename);
                    break :blk try FileResource.open(filename);
                } else return err,
                else => return err,
            }
        };

        if (mode.write) {
            if (mode.access_flag) {
                return error.AlreadyExists;
            }
            fd.truncate(0) catch {};
        }

        return .{ .resource = fd, .mode = mode };
    }

    pub fn offset_add(self: *Self, amount: isize) void {
        if (self.offset >= 0) {
            self.offset += amount;
            self.offset = @max(0, self.offset);
        } else if (self.offset != -1) {
            // -1 is from end
            // -2 is 1 byte before end
            // so we want to add (amount - 1) and make sure it maximumally reaches the end
            // FIXME: overthrow this system
            self.offset += @intCast(amount - 1);
            if (self.offset >= 0) self.offset = -1;
        }
    }

    pub fn set_offset(self: *Self, offset: isize) void {
        self.offset = offset;
    }

    pub fn read(self: *Self, buffer: []u8) errors.Error!usize {
        std.debug.assert(self.mode.read);
        const amount = self.resource.read(self.offset, buffer) catch |err| {
            switch (err) {
                error.InvaildOffset, error.OperationNotSupported => return 0,
                else => return err,
            }
        };
        self.offset_add(@intCast(amount));
        return amount;
    }

    pub fn write(self: *Self, buffer: []const u8) errors.Error!usize {
        std.debug.assert(self.mode.write);

        const amount = self.resource.write(self.offset, buffer) catch |err| {
            switch (err) {
                error.InvaildOffset, error.OperationNotSupported => return 0,
                else => return err,
            }
        };
        self.offset_add(@intCast(amount));
        return amount;
    }

    pub fn truncate(self: *Self, size: usize) errors.Error!void {
        std.debug.assert(self.mode.write);
        try self.resource.truncate(size);
        self.offset = @intCast(size);
    }

    pub fn sync(self: *Self) errors.Error!void {
        try self.resource.sync();
    }

    pub fn close(self: *Self) void {
        self.resource.close();
    }

    pub fn fsize(self: *Self) !usize {
        return self.resource.size();
    }
};

pub const GenericFileReader = std.io.GenericReader(*File, errors.Error, File.read);
pub const GenericFileWriter = std.io.GenericWriter(*File, errors.Error, File.write);

/// A file that's reads and writes on are instant
pub const InstantFile = struct {
    reader_inner: GenericFileReader,
    writer_inner: GenericFileWriter,
    const Self = @This();

    pub fn init(file: *File) InstantFile {
        return .{
            .reader_inner = .{ .context = file },
            .writer_inner = .{ .context = file },
        };
    }

    pub fn write(self: *Self, buffer: []const u8) errors.Error!usize {
        return self.writer_inner.write(buffer);
    }

    pub fn read(self: *Self, buffer: []u8) errors.Error!usize {
        return self.reader_inner.read(buffer);
    }
};

/// a File that is buffered until newline is encountered
pub const BufferedLineFile = struct {
    reader_inner: std.io.BufferedReader(4096, GenericFileReader),
    writer_inner: std.io.FindByteWriter(std.io.BufferedWriter(4096, GenericFileWriter)),

    const Self = @This();

    pub fn init(file: *File) BufferedLineFile {
        return .{
            .reader_inner = std.io.bufferedReader(GenericFileReader{ .context = file }),
            .writer_inner = std.io.findByteWriter('\n', std.io.bufferedWriter(GenericFileWriter{ .context = file })),
        };
    }

    pub fn write(self: *Self, buffer: []const u8) errors.Error!usize {
        const amount = try self.writer_inner.writer().write(buffer);
        if (self.writer_inner.byte_found) try self.flush();
        return amount;
    }

    pub fn read(self: *Self, buffer: []u8) errors.Error!usize {
        return self.reader_inner.read(buffer);
    }

    pub fn flush(self: *Self) errors.Error!void {
        try self.writer_inner.underlying_writer.flush();
    }
};

/// A file that's reads and writes are buffered
pub const BufferedFile = struct {
    reader_inner: std.io.BufferedReader(4096, GenericFileReader),
    writer_inner: std.io.BufferedWriter(4096, GenericFileWriter),
    const Self = @This();

    pub fn init(file: *File) BufferedFile {
        return .{
            .reader_inner = std.io.bufferedReader(GenericFileReader{ .context = file }),
            .writer_inner = std.io.bufferedWriter(GenericFileWriter{ .context = file }),
        };
    }

    pub fn read(self: *Self, buffer: []u8) errors.Error!usize {
        return self.reader_inner.read(buffer);
    }

    pub fn write(self: *Self, buffer: []const u8) errors.Error!usize {
        return self.writer_inner.write(buffer);
    }

    pub fn flush(self: *Self) errors.Error!void {
        try self.writer_inner.flush();
    }
};

pub const BufferingOption = enum(u2) {
    None = 0,
    Buffered = 1,
    LineBuffered = 2,
};

/// A file with multiple reads and writes buffering options.
pub const GenericFile = struct {
    // TODO: add custom buffering
    const FileBuffering = union(enum) {
        None: InstantFile,
        Buffered: BufferedFile,
        LineBuffered: BufferedLineFile,

        pub const Default = .Buffered;
    };

    file: *File,
    eof: bool = false,
    allocator: std.mem.Allocator,
    buffering: FileBuffering,

    pub const Reader = std.io.GenericReader(*Self, errors.Error, Self.read);
    pub const Writer = std.io.GenericWriter(*Self, errors.Error, Self.write);

    const Self = @This();

    /// Wraps a file resource into a GenericFile instance with the specified buffering option.
    pub fn fromResource(resource_id: io.FileResource, buffering: BufferingOption, mode: ModeFlags, allocator: std.mem.Allocator) !Self {
        const file = try allocator.create(File);
        file.* = .{ .resource = resource_id, .mode = mode };

        return Self{
            .file = file,
            .allocator = allocator,
            .buffering = switch (buffering) {
                .None => .{ .None = InstantFile.init(file) },
                .Buffered => .{ .Buffered = BufferedFile.init(file) },
                .LineBuffered => .{ .LineBuffered = BufferedLineFile.init(file) },
            },
        };
    }

    pub fn open(filename: []const u8, mode: ModeFlags, buffering: BufferingOption, allocator: std.mem.Allocator) errors.Error!*Self {
        const file = try File.open(filename, mode);
        const ptr = try allocator.create(File);
        ptr.* = file;

        const this = Self{
            .file = ptr,
            .allocator = allocator,
            .buffering = switch (buffering) {
                .None => .{ .None = InstantFile.init(ptr) },
                .Buffered => .{ .Buffered = BufferedFile.init(ptr) },
                .LineBuffered => .{ .LineBuffered = BufferedLineFile.init(ptr) },
            },
        };

        const this_ptr = try allocator.create(Self);
        this_ptr.* = this;
        return this_ptr;
    }
    pub fn set_buffering(self: *Self, buffering: BufferingOption) void {
        try self.flush();

        self.buffering = switch (buffering) {
            .None => .{ .None = InstantFile.init(self.file) },
            .Buffered => .{ .Buffered = BufferedFile.init(self) },
            .LineBuffered => .{ .LineBuffered = BufferedLineFile.init(self) },
        };
    }

    pub fn read(self: *Self, buffer: []u8) errors.Error!usize {
        const amount = try switch (self.buffering) {
            inline else => |*buffered| buffered.read(buffer),
        };

        self.eof = amount == 0;
        return amount;
    }

    pub fn write(self: *Self, buffer: []const u8) errors.Error!usize {
        switch (self.buffering) {
            inline else => |*buffered| return buffered.write(buffer),
        }
    }

    pub fn reader(self: *Self) Reader {
        return .{ .context = self };
    }

    pub fn writer(self: *Self) Writer {
        return .{ .context = self };
    }

    pub fn flush(self: *Self) errors.Error!void {
        switch (self.buffering) {
            .None => {},
            inline else => |*buffered| try buffered.flush(),
        }

        try self.file.sync();
    }

    pub fn closeChecked(self: *Self) errors.Error!void {
        defer self.allocator.destroy(self);
        defer self.allocator.destroy(self.file);

        try self.flush();
        self.file.close();
    }

    pub fn close(self: *Self) void {
        self.closeChecked() catch {};
    }
};
