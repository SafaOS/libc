const syscalls = @import("syscalls.zig");
const errors = @import("errno.zig");
const stdio = @import("../stdio.zig");
pub const raw = @import("raw.zig");

export fn open(path: *const u8, len: usize) isize {
    const path_buf: [*]const u8 = @ptrCast(path);

    const fd = zopen(path_buf[0..len]) catch |err| {
        const errno: u16 = @intFromError(err);
        errors.errno = @intCast(errno);
        return -1;
    };
    return @bitCast(fd);
}

export fn close(fd: isize) isize {
    zclose(@bitCast(fd)) catch |err| {
        const errno: u16 = @intFromError(err);
        errors.errno = @intCast(errno);
        return -1;
    };
    return 0;
}

export fn diriter_open(dir: usize) isize {
    const diriter = zdiriter_open(dir) catch |err| {
        const errno: u16 = @intFromError(err);
        errors.errno = @intCast(errno);
        return -1;
    };
    return @bitCast(diriter);
}

export fn diriter_close(diriter: isize) isize {
    zdiriter_close(@bitCast(diriter)) catch |err| {
        errors.seterr(err);
        return -1;
    };

    return 0;
}

export fn read(fd: usize, offset: isize, ptr: *u8, size: usize) isize {
    const buffer: [*]u8 = @ptrCast(ptr);
    const result = zread(fd, offset, buffer[0..size]) catch |err| {
        const errno: u16 = @intFromError(err);
        errors.errno = @intCast(errno);
        return -1;
    };

    return @bitCast(result);
}

export fn write(fd: usize, offset: isize, ptr: *const u8, size: usize) isize {
    const buffer: [*]const u8 = @ptrCast(ptr);
    const result = zwrite(fd, offset, buffer[0..size]) catch |err| {
        const errno: u16 = @intFromError(err);
        errors.errno = @intCast(errno);
        return -1;
    };
    return @bitCast(result);
}

pub export fn create(path: *const u8, len: usize) isize {
    const path_buf: [*]const u8 = @ptrCast(path);
    zcreate(path_buf[0..len]) catch |err| {
        const errno: u16 = @intFromError(err);
        errors.errno = @intCast(errno);
        return -1;
    };

    return 0;
}

pub export fn createdir(path_ptr: [*]const u8, len: usize) isize {
    const path = path_ptr[0..len];
    zcreatedir(path) catch |err| {
        errors.seterr(err);
        return -1;
    };

    return 0;
}

/// Opens a file and returns a file descriptor resource identifier
pub fn zopen(path: []const u8) errors.Error!usize {
    var fd: usize = undefined;
    try syscalls.open(@ptrCast(path.ptr), path.len, &fd).into_err();
    return fd;
}

pub fn zclose(fd: usize) errors.Error!void {
    try syscalls.close(@bitCast(fd)).into_err();
}

pub fn zdiriter_open(dir: usize) errors.Error!usize {
    var dir_ri: usize = undefined;
    try syscalls.diriter_open(dir, &dir_ri).into_err();
    return dir_ri;
}

pub fn zdiriter_close(diriter: usize) errors.Error!void {
    try syscalls.diriter_close(diriter).into_err();
}

pub fn zdiriter_next(diriter: usize) ?raw.DirEntry {
    var entry: raw.DirEntry = undefined;
    syscalls.diriter_next(diriter, &entry).into_err() catch return null;
    if (entry.name_length == 0 and entry.size == 0 and entry.kind == 0)
        return null;

    return entry;
}

pub fn zread(fd: usize, offset: isize, buffer: []u8) errors.Error!usize {
    var bytes_read: usize = undefined;

    try syscalls.read(fd, offset, @ptrCast(buffer.ptr), buffer.len, &bytes_read).into_err();
    return bytes_read;
}

pub fn zwrite(fd: usize, offset: isize, buffer: []const u8) errors.Error!usize {
    var bytes_wrote: usize = undefined;

    try syscalls.write(fd, offset, @ptrCast(buffer.ptr), buffer.len, &bytes_wrote).into_err();
    return bytes_wrote;
}

pub fn zcreate(path: []const u8) errors.Error!void {
    try syscalls.create(@ptrCast(path.ptr), path.len).into_err();
}

pub fn zcreatedir(path: []const u8) errors.Error!void {
    try syscalls.createdir(path.ptr, path.len).into_err();
}

pub export fn chdir(path_ptr: [*]const u8, path_len: usize) isize {
    const path = path_ptr[0..path_len];
    zchdir(path) catch |err| {
        errors.seterr(err);
        return -1;
    };
    return 0;
}

pub export fn getcwd(ptr: [*]u8, len: usize) isize {
    const buffer = ptr[0..len];
    const glen = zgetcwd(buffer) catch |err| {
        errors.seterr(err);
        return -1;
    };
    return @bitCast(glen);
}

pub fn zgetcwd(buffer: []u8) errors.Error!usize {
    var dest_len: usize = undefined;
    try syscalls.getcwd(buffer.ptr, buffer.len, &dest_len).into_err();
    return dest_len;
}

pub fn zchdir(path: []const u8) errors.Error!void {
    try syscalls.chdir(path.ptr, path.len).into_err();
}

pub fn zsync(ri: usize) errors.Error!void {
    try syscalls.sync(ri).into_err();
}

pub fn ztruncate(ri: usize, len: usize) errors.Error!void {
    try syscalls.truncate(ri, len).into_err();
}

pub fn zctl(ri: usize, cmd: u16, args: []usize) errors.Error!void {
    try syscalls.ctl(ri, cmd, args.ptr, args.len).into_err();
}

/// NOTE: for now we only take 3 arguments and pass them to the syscall as a list of usizes and don't really care about varargs
/// FIXME: I realized that ioctl isn't actually varargs it is defined that way for simplicity...
export fn ioctl(ri: usize, cmd: u16, ...) c_int {
    var list = @cVaStart();
    var args: [3]usize = undefined;
    const a = @cVaArg(&list, usize);
    const b = @cVaArg(&list, usize);
    const c = @cVaArg(&list, usize);
    args[0] = a;
    args[1] = b;
    args[2] = c;
    @cVaEnd(&list);
    zctl(ri, cmd, &args) catch |err| {
        errors.seterr(err);
        return -1;
    };
    return 0;
}
