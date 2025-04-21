const abi = @import("../../abi/root.zig");
const SysResult = abi.errors.SysError;
const RawSlice = abi.ffi.RawSlice;
const Optional = abi.ffi.Optional;
const DirEntry = abi.ffi.DirEntry;
const FileAttrs = abi.ffi.FileAttr;

pub const Pid = usize;
pub const Ri = usize;

pub extern fn syscreate_file(path_ptr: [*]const u8, path_len: usize) SysResult;
pub extern fn syscreatedir(path_ptr: [*]const u8, path_len: usize) SysResult;

pub extern fn sysopen(path_ptr: [*]const u8, path_len: usize, dest_fd: *Ri) SysResult;
pub extern fn sysdiriter_open(dir_ri: Ri, dest_iter_ri: *Ri) SysResult;
pub extern fn sysclose(fd: Ri) SysResult;
pub extern fn sysdiriter_close(iter_ri: Ri) SysResult;

pub extern fn sysread(fd: Ri, offset: isize, dest_ptr: [*]u8, dest_len: usize, dest_read: ?*usize) SysResult;
pub extern fn syswrite(fd: Ri, offset: isize, src_ptr: [*]const u8, src_len: usize, dest_written: ?*usize) SysResult;
pub extern fn systruncate(fd: Ri, new_len: usize) SysResult;
pub extern fn syssync(fd: Ri) SysResult;
pub extern fn sysdiriter_next(iter_ri: Ri, dest_entry: *DirEntry) SysResult;

pub extern fn sysdup(fd: Ri, dest_fd: *Ri) SysResult;
pub extern fn sysfattrs(fd: Ri, dest_attrs: *FileAttrs) SysResult;
pub extern fn sysfsize(fd: Ri, dest_size: *usize) SysResult;

pub extern fn syschdir(path_ptr: [*]const u8, path_len: usize) SysResult;
pub extern fn sysgetcwd(dest_ptr: [*]u8, dest_len: usize, dest_cwd_len: ?*usize) SysResult;
pub extern fn sysgetdirentry(path_ptr: [*]const u8, path_len: usize, dest_entry: ?*DirEntry) SysResult;

pub extern fn sysyield() SysResult;
pub extern fn syswait(pid: Pid, dest_status: ?*usize) SysResult;
pub extern fn syssbrk(size: isize, dest_brk: *usize) SysResult;

pub extern fn syspspawn(
    name_ptr: ?[*]const u8,
    name_len: usize,
    path_ptr: [*]const u8,
    path_len: usize,
    argv_ptr: ?[*]const RawSlice(u8),
    argv_len: usize,
    flags: u8,
    dest_pid: *Pid,
    stdin: Optional(Ri),
    stdout: Optional(Ri),
    stderr: Optional(Ri),
) SysResult;
pub extern fn sysexit(code: usize) noreturn;
