use core::ffi::{CStr, c_char, c_int};

use alloc::boxed::Box;
use safa_api::{
    abi::{
        consts::MAX_NAME_LENGTH,
        fs::{FSObjectType, OpenOptions},
    },
    errors::ErrorStatus,
    syscalls::{self, types::Ri},
};

use crate::{errno::set_error, file::File, try_errno};

pub fn entry_name(entry: &DirEnt) -> &str {
    CStr::from_bytes_until_nul(&entry.d_name)
        .expect("DirEnt isn't null terminated")
        .to_str()
        .expect("DirEnt isn't utf8")
}

#[repr(C)]
#[derive(Debug)]
pub struct DirEnt {
    pub d_name: [u8; MAX_NAME_LENGTH + 1],
}

#[derive(Debug)]
pub struct Dir {
    ri: Ri,
    curr_index: usize,
    entry: DirEnt,
}

impl Dir {
    pub fn open(path: &str) -> Result<Self, ErrorStatus> {
        let f = File::open(path, OpenOptions::from_bits(0))?;
        let ri = f.open_diriter()?;
        Ok(Self {
            ri,
            curr_index: 0,
            entry: unsafe { core::mem::zeroed() },
        })
    }

    pub fn next(&mut self) -> Option<&DirEnt> {
        let e = syscalls::io::diriter_next(self.ri).ok()?;
        self.curr_index += 1;
        self.entry.d_name[..e.name.len()].copy_from_slice(&e.name);
        self.entry.d_name[e.name_length] = 0;
        Some(&self.entry)
    }

    pub fn close(self) -> Result<(), ErrorStatus> {
        let ri = self.ri;
        core::mem::forget(self);
        syscalls::resources::destroy(ri)
    }

    pub unsafe fn close_ref(&mut self) -> Result<(), ErrorStatus> {
        syscalls::resources::destroy(self.ri)
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        unsafe { self.close_ref().expect("Failed to close diriter") }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mkdir(path: *const c_char, mode: u32) -> c_int {
    _ = mode;
    let c_str = unsafe { CStr::from_ptr(path) };
    let Ok(path) = c_str.to_str() else {
        set_error(ErrorStatus::InvalidPath);
        return -1;
    };
    try_errno!(File::open(path, OpenOptions::CREATE_DIRECTORY), -1);
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn rmdir(path: *const c_char) -> c_int {
    let c_str = unsafe { CStr::from_ptr(path) };
    let Ok(path) = c_str.to_str() else {
        set_error(ErrorStatus::InvalidPath);
        return -1;
    };

    if try_errno!(syscalls::fs::getdirentry(path), -1).attrs.kind != FSObjectType::Directory {
        set_error(ErrorStatus::NotADirectory);
        return -1;
    }
    try_errno!(syscalls::fs::remove_path(path), -1);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn opendir(path: *const c_char) -> *mut Dir {
    let c_str = unsafe { CStr::from_ptr(path) };
    let Ok(path) = c_str.to_str() else {
        return core::ptr::null_mut();
    };

    let opened = try_errno!(Dir::open(path), core::ptr::null_mut());
    Box::leak(Box::new(opened))
}

#[unsafe(no_mangle)]
pub extern "C" fn readdir(dir: *mut Dir) -> *const DirEnt {
    let dir = unsafe { &mut *dir };
    if let Some(e) = dir.next() {
        e
    } else {
        core::ptr::null()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn telldir(dir: *mut Dir) -> c_int {
    unsafe { (*dir).curr_index as c_int }
}

#[unsafe(no_mangle)]
pub extern "C" fn closedir(dir: *mut Dir) -> c_int {
    unsafe {
        let boxed = Box::from_raw(dir);
        try_errno!(boxed.close(), -1);
        0
    }
}
