use core::ffi::{CStr, c_char, c_int};

use safa_api::{abi::fs::FSObjectType, errors::ErrorStatus, syscalls};

use crate::{errno::set_error, try_errno};

pub type Mode = u32;
pub const S_IFMT: Mode = 0o170000; // Bit mask for the file type bit field
pub const S_IFDIR: Mode = 0o040000; // File type value for a directory
pub const S_IFREG: Mode = 0o100000;
pub const S_IFCHR: Mode = 0o020000;

#[repr(C)]
pub struct Stat {
    pub st_mode: Mode,
    pub st_size: i64,
    pub st_atime: u64,
    pub st_mtime: u64,
    pub st_ctime: u64,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn stat(path: *const c_char, stat_buf: *mut Stat) -> c_int {
    let path = unsafe {
        let Ok(p) = CStr::from_ptr(path).to_str() else {
            set_error(ErrorStatus::InvalidPath);
            return -1;
        };
        p
    };

    let ent = try_errno!(syscalls::fs::getdirentry(path), -1);
    let mut st_mode = 0o777;
    match ent.attrs.kind {
        FSObjectType::Directory => {
            st_mode |= S_IFDIR;
        }
        FSObjectType::File => {
            st_mode |= S_IFREG;
        }
        FSObjectType::Device => {
            st_mode |= S_IFCHR;
        }
    }

    unsafe {
        if !stat_buf.is_null() {
            *stat_buf = Stat {
                st_mode,
                st_size: ent.attrs.size as i64,
                // TODO: Implement timing metadata
                st_ctime: 0,
                st_mtime: 0,
                st_atime: 0,
            };
        }
    }
    0
}
