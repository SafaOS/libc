const sys = @import("sys/root.zig");
const api = sys.api;
const abi = sys.abi;

const syscalls = api.syscalls;
const alloc = api.alloc;

const errors = abi.errors;

const string = @import("string.zig");
const extra = @import("extra.zig");
const panic = @import("root.zig").panic;
const stdlib = @import("stdlib.zig");
const geterr = errors.geterr;
const seterr = errors.seterr;
const std = @import("std");
const allocator = stdlib.c_allocator;

const EOF: u8 = 255;

const VaList = @import("std").builtin.VaList;

/// TODO: remove is a stub
export fn remove(path: [*:0]const c_char) c_int {
    _ = path;
    std.debug.panic("remove(path: [*:0]const c_char) is not yet implemented", .{});
}

/// TODO: rename is a stub
export fn rename(oldpath: [*:0]const c_char, newpath: [*:0]const c_char) c_int {
    _ = oldpath;
    _ = newpath;
    std.debug.panic("rename(oldpath: [*:0]const c_char, newpath: [*:0]const c_char) is not yet implemented", .{});
}

const io = api.io;
const ModeFlags = io.ModeFlags;

pub const SeekWhence = enum(usize) {
    Set = 0,
    Current = 1,
    End = 2,
};

const BufferingOption = io.BufferingOption;
const c_allocator = stdlib.c_allocator;

const FILE = io.GenericFile;

pub fn open(filename: []const u8, mode: ModeFlags) errors.Error!*FILE {
    return io.GenericFile.open(filename, mode, .LineBuffered, c_allocator);
}

pub fn seek(self: *FILE, offset: isize, whence: SeekWhence) errors.Error!void {
    switch (whence) {
        .Set => self.file.set_offset(offset),
        .Current => self.file.offset_add(offset),
        .End => self.file.set_offset(-1 - offset),
    }
}

/// helper to write a formatted string to a writer in a C-style format
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

/// writes a formatted string a writern in a C-style format
pub fn writeVarFmt(self: std.io.AnyWriter, fmt: [*:0]const u8, args: *VaList) !void {
    var current = fmt;

    while (current[0] != 0) {
        current = try traverseFmt(self, current) orelse return;
        current += 1;

        const Fmt = packed struct {
            const Kind = enum(u3) {
                none,
                dec,
                long,
                size_t,
                // null terminated string literal
                string,
                // string pointer and length
                sized_string,
            };

            const Flags = packed struct(u3) {
                unsigned: bool = false,
                hex: bool = false,
                big_hex: bool = false,
            };
            kind: Kind = .none,
            flags: Flags = .{},

            // Sets the kind of self to `kind` returns whether or not to stop paring the fmt
            fn setKind(this: *@This(), kind: Kind) bool {
                return if (this.kind == .none) blk: {
                    this.kind = kind;
                    break :blk false;
                } else true;
            }

            fn print_int(this: @This(), parent: @TypeOf(self), comptime T: type, comptime unsigned_T: type, value: T) !void {
                if (this.flags.hex)
                    return if (this.flags.unsigned) parent.print("{x}", .{@as(unsigned_T, @bitCast(value))}) else parent.print("{x}", .{value});
                if (this.flags.big_hex)
                    return if (this.flags.unsigned) parent.print("{X}", .{@as(unsigned_T, @bitCast(value))}) else parent.print("{X}", .{value});

                return if (this.flags.unsigned) parent.print("{}", .{@as(unsigned_T, @bitCast(value))}) else parent.print("{}", .{value});
            }

            fn print_int_va(this: @This(), parent: @TypeOf(self), args_list: @TypeOf(args), comptime T: type, unsigned_T: type) !void {
                const value = @cVaArg(args_list, T);
                return this.print_int(parent, T, unsigned_T, value);
            }
        };

        var spec: Fmt = .{};

        while (current[0] != 0) {
            const should_stop = switch (current[0]) {
                'd' => spec.setKind(.dec),
                'z' => spec.setKind(.size_t),
                'u' => blk: {
                    _ = spec.setKind(.dec);
                    spec.flags.unsigned = true;
                    break :blk false;
                },
                'l' => spec.setKind(.long),
                'p' => blk: {
                    if (spec.setKind(.size_t)) break :blk true;
                    spec.flags.unsigned = true;
                    spec.flags.hex = true;
                    break :blk false;
                },
                'x' => blk: {
                    _ = spec.setKind(.dec);
                    spec.flags.hex = true;
                    break :blk false;
                },
                'X' => blk: {
                    _ = spec.setKind(.dec);
                    spec.flags.big_hex = true;
                    break :blk false;
                },
                's' => spec.setKind(.string),
                '.' => switch (current[0]) {
                    '*' => blk: {
                        current += 1;
                        break :blk switch (current[0]) {
                            's' => spec.setKind(.sized_string),
                            else => true,
                        };
                    },
                    else => true,
                },

                else => true,
            };
            if (should_stop) break else current += 1;
        }

        switch (spec.kind) {
            .string => {
                const arg = @cVaArg(args, [*:0]const u8);
                try self.print("{s}", .{arg});
            },
            .sized_string => {
                const len = @cVaArg(args, usize);
                const str = @cVaArg(args, [*c]const u8);
                try self.print("{s}", .{str[0..len]});
            },
            .dec => try spec.print_int_va(self, args, c_int, c_uint),
            .long => try spec.print_int_va(self, args, c_long, c_ulong),
            .size_t => try spec.print_int_va(self, args, isize, usize),
            .none => {
                continue;
            },
        }
    }
}

pub var stdin_desc: ?FILE = null;
pub var stdout_desc: ?FILE = null;
pub var stderr_desc: ?FILE = null;

pub export var stdin: *FILE = undefined;
pub export var stdout: *FILE = undefined;
pub export var stderr: *FILE = undefined;

pub fn init_stdin() *FILE {
    stdin_desc = FILE.fromResource(syscalls.io.stdin(), .Buffered, .{ .read = true }, c_allocator) catch unreachable;
    return &stdin_desc.?;
}

pub fn init_stdout() *FILE {
    stdout_desc = FILE.fromResource(syscalls.io.stdout(), .LineBuffered, .{ .write = true }, c_allocator) catch unreachable;
    return &stdout_desc.?;
}

pub fn init_stderr() *FILE {
    stderr_desc = FILE.fromResource(syscalls.io.stderr(), .None, .{ .write = true }, c_allocator) catch unreachable;
    return &stderr_desc.?;
}

export fn fopen(filename: [*:0]const c_char, mode: [*:0]const c_char) ?*FILE {
    const path: [*:0]const u8 = @ptrCast(filename);
    const len = string.strlen(filename);
    const modeflags = ModeFlags.from_cstr(mode) orelse {
        seterr(error.InvaildStr);
        return null;
    };

    return open(path[0..len], modeflags) catch |err| {
        seterr(err);
        return null;
    };
}

// TODO: tmpfile is a stub
// because file deleting is not yet a thing
export fn tmpfile() *FILE {
    std.debug.panic("tmpfile(): not yet implemented", .{});
}

export fn tmpnam(s: [*c]u8) [*c]u8 {
    _ = s;
    std.debug.panic("tmpnam: is not yet implemented", .{});
}

// TODO: add Custom buffering
export fn setvbuf(file: *FILE, custom_buffer: [*c]u8, mode: BufferingOption, size: usize) c_int {
    std.debug.assert(custom_buffer == null);
    std.debug.assert(size == 4096 or size == 0);

    file.set_buffering(mode) catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}

// TODO: partial stub
export fn freopen(filename: [*:0]const c_char, mode: [*:0]const c_char, file: *FILE) ?*FILE {
    _ = fclose(file);
    return fopen(filename, mode);
}

export fn feof(stream: *FILE) c_int {
    return @intFromBool(stream.eof);
}

/// TODO: clearerr is a stub
export fn clearerr(stream: *FILE) void {
    _ = stream;
}

/// TODO: ferror is a stub
export fn ferror(stream: *FILE) c_int {
    _ = stream;
    return 0;
}

export fn fclose(file: *FILE) c_int {
    file.closeChecked() catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}

export fn fread(ptr: [*]u8, size: usize, count: usize, stream: *FILE) usize {
    const amount = stream.reader().read(ptr[0 .. size * count]) catch |err| {
        seterr(@errorCast(err));
        return 0;
    };
    return amount / size;
}

export fn fwrite(ptr: [*]const u8, size: usize, count: usize, stream: *FILE) usize {
    const amount = stream.writer().write(ptr[0 .. size * count]) catch |err| {
        seterr(@errorCast(err));
        return 0;
    };
    return amount / size;
}

fn zfgetc(stream: *FILE) errors.Error!u8 {
    return @errorCast(stream.reader().readByte());
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

// TODO: implement ungetc when different file buffering options are available
export fn ungetc(c: c_int, stream: *FILE) c_int {
    _ = c;
    _ = stream;
    std.debug.panic("ungetc: is not yet implemented", .{});
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

pub fn zerrprintf(comptime fmt: []const u8, args: anytype) void {
    const writer = stderr.writer();
    writer.print(fmt, args) catch {};
}

pub fn zprintf(comptime fmt: []const u8, args: anytype) void {
    const writer = stdout.writer();
    writer.print(fmt, args) catch {};
}

export fn fprintf(stream: *FILE, fmt: [*:0]const u8, ...) c_int {
    var args = @cVaStart();
    var writer = stream.writer();
    writeVarFmt(writer.any(), fmt, &args) catch |err| {
        seterr(@errorCast(err));
        return -1;
    };
    return 0;
}

inline fn pseudo_print(fmt: [*:0]const u8, args: *VaList) !usize {
    var stream = std.io.countingWriter(std.io.null_writer);
    const writer = stream.writer();
    try writeVarFmt(writer.any(), fmt, args);

    return @bitCast(stream.bytes_written);
}

/// Writes the formatted string to the buffer in a C-style format.
/// Returns the number of bytes written to the buffer.
///
/// buffer can be null, in which case the number of bytes that would have been written is returned.
inline fn zsnprintf(buffer: ?[]u8, fmt: [*:0]const u8, args: *VaList) !usize {
    if (buffer == null)
        return pseudo_print(fmt, args);

    var stream = std.io.fixedBufferStream(buffer.?);
    var writer = stream.writer();
    try writeVarFmt(writer.any(), fmt, args);

    return stream.pos;
}

export fn snprintf(str: ?[*:0]u8, size: usize, fmt: [*:0]const u8, ...) c_int {
    var args = @cVaStart();

    const buffer = if (str) |buf| buf[0 .. size - 1] else null;

    const len = zsnprintf(buffer, fmt, &args) catch |err| {
        seterr(@errorCast(err));
        return -1;
    };

    if (str) |buf| buf[len] = 0;
    return @intCast(len);
}

export fn sprintf(str: [*]u8, fmt: [*:0]const u8, ...) c_int {
    var args = @cVaStart();

    const len = zsnprintf(str[0..std.math.maxInt(usize)], fmt, &args) catch |err| {
        seterr(@errorCast(err));
        return -1;
    };

    str[len] = 0;
    return @intCast(len);
}

export fn printf(fmt: [*:0]const u8, ...) c_int {
    var args = @cVaStart();
    var writer = stdout.writer();
    writeVarFmt(writer.any(), fmt, &args) catch |err| {
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
    stream.writer().writeByte(c) catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}

export fn fputs(str: [*:0]const u8, stream: *FILE) c_int {
    _ = FILE.write(stream, str[0..std.mem.len(str)]) catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}

export fn fgets(str: [*]u8, size: usize, stream: *FILE) ?[*]u8 {
    const amount = FILE.read(stream, str[0 .. size - 1]) catch |err| {
        seterr(err);
        return null;
    };

    str[amount] = 0;
    return str;
}

export fn fseek(stream: *FILE, offset: isize, whence: c_int) c_int {
    if (whence > @intFromEnum(SeekWhence.End)) {
        seterr(error.Generic);
        return -1;
    }
    const seek_whence: SeekWhence = @enumFromInt(whence);

    seek(stream, offset, seek_whence) catch |err| {
        seterr(err);
        return -1;
    };
    return 0;
}

export fn ftell(stream: *FILE) usize {
    if (stream.file.offset >= 0) {
        return @bitCast(stream.file.offset);
    } else {
        const size = stream.file.fsize() catch unreachable;
        const pos_from_end: usize = @bitCast(-stream.file.offset - 1);
        return size - pos_from_end;
    }
}
