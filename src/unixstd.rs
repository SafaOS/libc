//! This is an emulation layer of unix file descriptors.
//!
//! Includes functions from unistd, fcntl...

use core::ffi::{CStr, c_char, c_int};

use safa_api::{
    abi::fs::{OpenOptions, SeekWrench},
    errors::ErrorStatus,
    resource::Resource,
    syscalls::{self, types::Ri},
};

use crate::{errno::set_error, file::SeekPosition, try_errno};

pub type FD = Ri;
const _: () = assert!(size_of::<FD>() == size_of::<c_int>());

#[unsafe(no_mangle)]
pub extern "C" fn read(fd: FD, buf: *mut u8, count: usize) -> isize {
    let slice = unsafe { core::slice::from_raw_parts_mut(buf, count) };
    try_errno!(syscalls::io::read_next(fd, slice), -1) as isize
}

#[unsafe(no_mangle)]
pub extern "C" fn write(fd: FD, buf: *const u8, count: usize) -> isize {
    let slice = unsafe { core::slice::from_raw_parts(buf, count) };
    try_errno!(syscalls::io::write_next(fd, slice), -1) as isize
}

#[unsafe(no_mangle)]
pub extern "C" fn lseek(fd: FD, offset: isize, whence: c_int) -> isize {
    let pos = match (whence, offset >= 0) {
        (0, true) => SeekPosition::Start(offset as usize),
        (0, false) => SeekPosition::End((-offset) as usize),
        (1, true | false) => SeekPosition::Current(offset as isize),
        (2, true) => SeekPosition::End(offset as usize),
        (2, false) => SeekPosition::Start((-offset) as usize),
        _ => return -1,
    };

    try_errno!(syscalls::io::seek(fd, pos.into()), -1) as isize
}

const O_APPEND: c_int = 1 << 0;
const O_CREAT: c_int = 1 << 1;
const O_EXCL: c_int = 1 << 2;
const O_TRUNC: c_int = 1 << 3;
const O_WRONLY: c_int = 1 << 4;
const O_RDONLY: c_int = 1 << 5;
const _O_RDWR: c_int = 1 << 6;

#[unsafe(no_mangle)]
pub extern "C" fn creat(path: *const c_char, _mode: c_int) -> c_int {
    let options = OpenOptions::CREATE_FILE | OpenOptions::WRITE_TRUNCATE | OpenOptions::WRITE;
    let cpath = unsafe { CStr::from_ptr(path) };
    let Ok(path) = cpath.to_str() else {
        set_error(ErrorStatus::InvalidPath);
        return -1;
    };

    let res = try_errno!(Resource::open(path, options), -1);
    let ri = res.ri();
    core::mem::forget(res);

    ri as c_int
}
#[unsafe(no_mangle)]
pub extern "C" fn open(path: *const c_char, flags: c_int, _mode: c_int) -> c_int {
    let cpath = unsafe { CStr::from_ptr(path) };
    let Ok(path) = cpath.to_str() else {
        set_error(ErrorStatus::InvalidPath);
        return -1;
    };

    let mut opts = OpenOptions::from_bits(0);

    if flags & O_CREAT != 0 {
        opts = opts | OpenOptions::CREATE_FILE;

        if flags & O_EXCL != 0 {
            if !matches!(
                Resource::open(path, OpenOptions::READ),
                Err(ErrorStatus::NoSuchAFileOrDirectory),
            ) {
                set_error(ErrorStatus::AlreadyExists);
                return -1;
            }
        }
    }

    if flags & O_RDONLY != 0 {
        opts = opts | OpenOptions::READ;
    } else if flags & O_WRONLY != 0 {
        opts = opts | OpenOptions::WRITE;
    } else {
        opts = opts | OpenOptions::READ | OpenOptions::WRITE;
    }

    if flags & O_TRUNC != 0 {
        opts = opts | OpenOptions::WRITE_TRUNCATE;
    }

    let res = try_errno!(Resource::open(path, opts), -1);
    let ri = res.ri();

    if flags & O_APPEND != 0 && flags & O_RDONLY == 0 {
        try_errno!(res.seek(SeekWrench::End(0)), -1);
    }
    core::mem::forget(res);

    ri as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn close(fd: FD) -> c_int {
    let resource = unsafe { Resource::from_raw(fd) };

    match resource.destroy() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dup(fd: FD) -> c_int {
    let resource = unsafe { Resource::from_raw(fd) };
    let clone = try_errno!(resource.clone(), -1);
    core::mem::forget(resource);

    let ri = clone.ri();
    core::mem::forget(clone);
    ri as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn unlink(path: *const c_char) -> c_int {
    let cpath = unsafe { CStr::from_ptr(path) };
    let Ok(path) = cpath.to_str() else {
        set_error(ErrorStatus::InvalidPath);
        return -1;
    };

    try_errno!(syscalls::fs::remove_path(path), -1);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn access(path: *const c_char, _mode: c_int) -> c_int {
    let cpath = unsafe { CStr::from_ptr(path) };
    let Ok(path) = cpath.to_str() else {
        set_error(ErrorStatus::InvalidPath);
        return -1;
    };

    try_errno!(syscalls::fs::getdirentry(path), -1);
    0
}
