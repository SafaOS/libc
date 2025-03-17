//! this file should contain the raw syscall functions
const raw = @import("raw.zig");
const SysError = @import("errno.zig").SysError;

inline fn syscall0(number: usize) SysError {
    return asm volatile ("int $0x80"
        : [ret] "={rax}" (-> usize),
        : [number] "{rax}" (number),
        : "rcx", "r11"
    );
}

inline fn syscall1(number: usize, arg1: usize) SysError {
    return asm volatile ("int $0x80"
        : [ret] "={rax}" (-> SysError),
        : [number] "{rax}" (number),
          [arg1] "{rdi}" (arg1),
        : "rcx", "r11"
    );
}

inline fn syscall3(number: usize, arg1: usize, arg2: usize, arg3: usize) SysError {
    return asm volatile ("int $0x80"
        : [ret] "={rax}" (-> SysError),
        : [number] "{rax}" (number),
          [arg1] "{rdi}" (arg1),
          [arg2] "{rsi}" (arg2),
          [arg3] "{rdx}" (arg3),
        : "rcx", "r11"
    );
}

inline fn syscall4(number: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) SysError {
    return asm volatile ("int $0x80"
        : [ret] "={rax}" (-> SysError),
        : [number] "{rax}" (number),
          [arg1] "{rdi}" (arg1),
          [arg2] "{rsi}" (arg2),
          [arg3] "{rdx}" (arg3),
          [arg4] "{rcx}" (arg4),
        : "r8", "r11"
    );
}

inline fn syscall5(number: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) SysError {
    return asm volatile ("int $0x80"
        : [ret] "={rax}" (-> SysError),
        : [number] "{rax}" (number),
          [arg1] "{rdi}" (arg1),
          [arg2] "{rsi}" (arg2),
          [arg3] "{rdx}" (arg3),
          [arg4] "{rcx}" (arg4),
          [arg5] "{r8}" (arg5),
        : "r9", "r11"
    );
}

inline fn syscall6(number: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize) SysError {
    return asm volatile ("int $0x80"
        : [ret] "={rax}" (-> usize),
        : [number] "{rax}" (number),
          [arg1] "{rdi}" (arg1),
          [arg2] "{rsi}" (arg2),
          [arg3] "{rdx}" (arg3),
          [arg4] "{rcx}" (arg4),
          [arg5] "{r8}" (arg5),
          [arg6] "{r9}" (arg6),
        : "r10", "r11"
    );
}
pub inline fn exit(code: usize) void {
    _ = syscall1(0, code);
}

pub inline fn yield() void {
    _ = syscall0(1);
}

pub inline fn open(path: *const u8, len: usize, fd: *usize) SysError {
    return syscall3(2, @intFromPtr(path), len, @intFromPtr(fd));
}

pub inline fn write(fd: usize, offset: isize, ptr: *const u8, len: usize, num_written: *usize) SysError {
    return syscall5(3, fd, @bitCast(offset), @intFromPtr(ptr), len, @intFromPtr(num_written));
}

pub inline fn read(fd: usize, offset: isize, ptr: *u8, len: usize, num_read: *usize) SysError {
    return syscall5(4, fd, @bitCast(offset), @intFromPtr(ptr), len, @intFromPtr(num_read));
}

pub inline fn close(fd: usize) SysError {
    return syscall1(5, fd);
}

pub inline fn create(path_ptr: *const u8, path_len: usize) SysError {
    return syscall3(6, @intFromPtr(path_ptr), path_len, 0);
}

pub inline fn createdir(path_ptr: [*]const u8, path_len: usize) SysError {
    return syscall3(7, @intFromPtr(path_ptr), path_len, 0);
}

pub inline fn diriter_open(dir_ri: usize, dest_diriter: *usize) SysError {
    return syscall3(8, dir_ri, @intFromPtr(dest_diriter), 0);
}

pub inline fn diriter_close(diriter: usize) SysError {
    return syscall1(9, diriter);
}

pub inline fn diriter_next(diriter: usize, direntry: *raw.DirEntry) SysError {
    return syscall3(10, diriter, @intFromPtr(direntry), 0);
}

pub inline fn wait(pid: usize, code: *usize) SysError {
    return syscall3(11, pid, @intFromPtr(code), 0);
}

pub inline fn ctl(ri: usize, cmd: u16, args_ptr: [*]const usize, args_len: usize) SysError {
    return syscall4(12, ri, cmd, @intFromPtr(args_ptr), args_len);
}

pub inline fn spawn(elf_ptr: *const u8, elf_len: usize, config: *const raw.SpawnConfig, dest_pid: *u64) SysError {
    return syscall4(13, @intFromPtr(elf_ptr), elf_len, @intFromPtr(config), @intFromPtr(dest_pid));
}

pub inline fn chdir(path_ptr: [*]const u8, path_len: usize) SysError {
    return syscall3(14, @intFromPtr(path_ptr), path_len, 0);
}

pub inline fn getcwd(ptr: [*]const u8, len: usize, dest_len_got: *usize) SysError {
    return syscall3(15, @intFromPtr(ptr), len, @intFromPtr(dest_len_got));
}

pub inline fn sync(ri: usize) SysError {
    return syscall1(16, ri);
}

pub inline fn truncate(ri: usize, len: usize) SysError {
    return syscall3(17, ri, len, 0);
}

pub inline fn sbrk(amount: isize, ptr: *usize) SysError {
    return syscall3(18, @bitCast(amount), @intFromPtr(ptr), 0);
}

pub inline fn pspawn(path_ptr: *const u8, path_len: usize, config: *const raw.SpawnConfig, dest_pid: *u64) SysError {
    return syscall4(19, @intFromPtr(path_ptr), path_len, @intFromPtr(config), @intFromPtr(dest_pid));
}

pub inline fn shutdown() noreturn {
    _ = syscall0(20);
    unreachable;
}

pub inline fn reboot() noreturn {
    _ = syscall0(21);
    unreachable;
}

pub inline fn fsize(fd: usize, dest_size: *usize) SysError {
    return syscall3(22, fd, @intFromPtr(dest_size), 0);
}
