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
    const err = syscalls.diriter_close(@bitCast(diriter));
    if (err != 0) {
        errors.errno = @truncate(err);
        return -1;
    }

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

pub export fn createdir(path: *const u8, len: usize) isize {
    const err = syscalls.createdir(path, len);
    if (err != 0) {
        errors.errno = @truncate(err);
        return -1;
    }

    return 0;
}

/// Opens a file and returns a file descriptor resource identifier
pub fn zopen(path: []const u8) errors.Error!usize {
    var fd: usize = undefined;

    const err: u16 = @truncate(syscalls.open(@ptrCast(path.ptr), path.len, &fd));
    if (err != 0) {
        const errno: errors.Error = @errorCast(@errorFromInt(err));
        return errno;
    }

    return fd;
}

pub fn zclose(fd: usize) errors.Error!void {
    const err: u16 = @truncate(syscalls.close(@bitCast(fd)));
    if (err != 0) {
        const errno: errors.Error = @errorCast(@errorFromInt(err));
        return errno;
    }
}

pub fn zdiriter_open(dir: usize) errors.Error!usize {
    var dir_ri: usize = undefined;
    const err: u16 = @truncate(syscalls.diriter_open(dir, &dir_ri));
    if (err != 0) {
        const errno: errors.Error = @errorCast(@errorFromInt(err));
        return errno;
    }

    return dir_ri;
}

pub fn zdiriter_close(diriter: usize) errors.Error!void {
    const err: u16 = @truncate(syscalls.diriter_close(diriter));
    if (err != 0) {
        const errno: errors.Error = @errorCast(@errorFromInt(err));
        return errno;
    }
}

pub fn zdiriter_next(diriter: usize) ?raw.DirEntry {
    var entry: raw.DirEntry = undefined;
    const err = syscalls.diriter_next(diriter, &entry);
    if (err != 0)
        return null;

    if (entry.name_length == 0 and entry.size == 0 and entry.kind == 0)
        return null;

    return entry;
}

pub fn zread(fd: usize, offset: isize, buffer: []u8) errors.Error!usize {
    var bytes_read: usize = undefined;

    const err: u16 = @truncate(syscalls.read(fd, offset, @ptrCast(buffer.ptr), buffer.len, &bytes_read));
    if (err != 0) {
        const errno: errors.Error = @errorCast(@errorFromInt(err));
        return errno;
    }

    return bytes_read;
}

pub fn zwrite(fd: usize, offset: isize, buffer: []const u8) errors.Error!usize {
    var bytes_wrote: usize = undefined;

    const err: u16 = @truncate(syscalls.write(fd, offset, @ptrCast(buffer.ptr), buffer.len, &bytes_wrote));
    if (err != 0) {
        const errno: errors.Error = @errorCast(@errorFromInt(err));
        return errno;
    }

    return bytes_wrote;
}

pub fn zcreate(path: []const u8) errors.Error!void {
    const errc: u16 = @truncate(syscalls.create(@ptrCast(path.ptr), path.len));

    if (errc != 0) {
        const err = @errorFromInt(errc);
        const errno: errors.Error = @errorCast(err);
        return errno;
    }
}

pub fn zcreatedir(path: []const u8) errors.Error!void {
    const err = createdir(@ptrCast(path.ptr), path.len);
    if (err == -1) return errors.geterr();
}

pub export fn chdir(path: [*]const u8, path_len: usize) isize {
    const err = syscalls.chdir(path, path_len);
    if (err != 0) {
        errors.errno = @truncate(err);
        return -1;
    }
    return 0;
}

pub export fn getcwd(ptr: [*]const u8, len: usize) isize {
    var dest_len: usize = undefined;
    const err = syscalls.getcwd(ptr, len, &dest_len);
    if (err != 0) {
        errors.errno = @truncate(err);
        return -1;
    }
    return @bitCast(dest_len);
}

pub fn zgetcwd(buffer: []u8) errors.Error!usize {
    const len = getcwd(@ptrCast(buffer.ptr), buffer.len);
    if (len == -1) return errors.geterr();
    return @bitCast(len);
}

pub fn zchdir(path: []const u8) errors.Error!void {
    const err = chdir(@ptrCast(path.ptr), path.len);
    if (err == -1) return errors.geterr();
}

pub fn zsync(ri: usize) errors.Error!void {
    const err: u16 = @truncate(syscalls.sync(ri));

    if (err != 0) {
        const err_t: errors.Error = @errorCast(@errorFromInt(err));
        return err_t;
    }
}

pub fn ztruncate(ri: usize, len: usize) errors.Error!void {
    const err: u16 = @truncate(syscalls.truncate(ri, len));
    if (err != 0) {
        const errno: errors.Error = @errorCast(@errorFromInt(err));
        return errno;
    }
}

pub fn zctl(ri: usize, cmd: u16, args: []usize) errors.Error!void {
    const err: u16 = @truncate(syscalls.ctl(ri, cmd, args.ptr, args.len));
    if (err != 0) {
        const errno: errors.Error = @errorCast(@errorFromInt(err));
        return errno;
    }
}

/// NOTE: for now we only take 3 arguments and pass them to the syscall as a list of usizes and don't really care about varargs
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
