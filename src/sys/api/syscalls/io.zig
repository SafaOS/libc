const raw = @import("raw.zig");
const abi = @import("../../abi/root.zig");
const DirEntry = abi.ffi.DirEntry;
const errors = abi.errors;
const SysError = abi.errors.SysError;

pub fn create(path: []const u8) errors.Error!void {
    try raw.syscreate_file(path.ptr, path.len).into_err();
}

pub fn createdir(path: []const u8) errors.Error!void {
    try raw.syscreatedir(path.ptr, path.len).into_err();
}

pub const FileResource = packed struct {
    const Self = @This();
    fd: raw.Ri,

    pub fn open(path: []const u8) errors.Error!Self {
        var fd: raw.Ri = undefined;
        try raw.sysopen(path.ptr, path.len, &fd).into_err();
        return .{ .fd = fd };
    }

    pub fn try_close(self: Self) errors.Error!void {
        try raw.sysclose(self.fd).into_err();
    }

    pub fn close(self: Self) void {
        self.try_close() catch unreachable;
    }

    pub fn read(self: Self, offset: isize, buf: []u8) errors.Error!usize {
        var read_amount: usize = undefined;
        try raw.sysread(self.fd, offset, buf.ptr, buf.len, &read_amount).into_err();
        return read_amount;
    }

    pub fn write(self: Self, offset: isize, buf: []const u8) errors.Error!usize {
        var written: usize = undefined;
        try raw.syswrite(self.fd, offset, buf.ptr, buf.len, &written).into_err();
        return written;
    }

    pub fn truncate(self: Self, new_len: usize) errors.Error!void {
        try raw.systruncate(self.fd, new_len).into_err();
    }

    pub fn sync(self: Self) errors.Error!void {
        try raw.syssync(self.fd).into_err();
    }

    pub fn dirIter(self: Self) errors.Error!DirIterResource {
        var iter_ri: raw.Ri = undefined;
        try raw.sysdiriter_open(self.fd, &iter_ri).into_err();
        return .{ .iter_ri = iter_ri, .parent = self };
    }

    pub fn size(self: Self) errors.Error!usize {
        var results: usize = undefined;
        try raw.sysfsize(self.fd, &results).into_err();
        return results;
    }
};

pub const DirIterResource = extern struct {
    const Self = @This();
    iter_ri: raw.Ri,
    parent: FileResource,

    fn try_close_self(self: Self) errors.Error!void {
        try raw.sysdiriter_close(self.iter_ri).into_err();
    }

    pub fn try_close(self: Self) errors.Error!void {
        try self.try_close_self();
        try self.parent.try_close();
    }

    pub fn close_self(self: Self) void {
        self.try_close_self() catch unreachable;
    }

    pub fn close(self: Self) void {
        self.try_close() catch unreachable;
    }

    pub fn next(self: Self) ?DirEntry {
        var entry: DirEntry = undefined;
        if (raw.sysdiriter_next(self.iter_ri, &entry) != SysError.None) return null;
        return if (entry.name_len == 0) null else entry;
    }
};

// TODO: ctl implementition
// example
// /// NOTE: for now we only take 3 arguments and pass them to the syscall as a list of usizes and don't really care about varargs
// // FIXME: I realized that ioctl isn't actually varargs it is defined that way for simplicity...
// export fn ioctl(ri: usize, cmd: u16, ...) c_int {
//     var list = @cVaStart();
//     var args: [3]usize = undefined;
//     const a = @cVaArg(&list, usize);
//     const b = @cVaArg(&list, usize);
//     const c = @cVaArg(&list, usize);
//     args[0] = a;
//     args[1] = b;
//     args[2] = c;
//     @cVaEnd(&list);
//     zctl(ri, cmd, &args) catch |err| {
//         errors.seterr(err);
//         return -1;
//     };
//     return 0;
// }

extern fn sysmeta_stdin() raw.Ri;
extern fn sysmeta_stdout() raw.Ri;
extern fn sysmeta_stderr() raw.Ri;

pub fn stdin() FileResource {
    return .{ .fd = sysmeta_stdin() };
}

pub fn stdout() FileResource {
    return .{ .fd = sysmeta_stdout() };
}

pub fn stderr() FileResource {
    return .{ .fd = sysmeta_stderr() };
}
