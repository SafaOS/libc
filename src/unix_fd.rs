//! This is an emulation layer of unix file descriptors.
//!
//! Includes functions from unistd, fcntl...

use core::{
    ffi::{CStr, c_char, c_int},
    sync::atomic::AtomicU16,
};

use hashbrown::HashMap;
use rustc_hash::FxBuildHasher;
use safa_api::{
    abi::fs::OpenOptions,
    errors::ErrorStatus,
    resource::Resource,
    sync::locks::Mutex,
    syscalls::{self, types::Ri},
};

use crate::{errno::set_error, file::SeekPosition, try_errno};

#[derive(Debug, Clone, Copy)]
pub struct Metadata {
    offset: isize,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FD {
    ri: u16,
    metadata: u16,
}
impl FD {
    pub fn with_meta<R>(&self, f: impl FnOnce(&mut Metadata) -> R) -> Option<R> {
        let mut meta = FAKE_FD_META.lock();

        let m = meta.get_mut(&self.metadata)?;
        Some(f(m))
    }
    pub fn size(&self) -> Result<usize, ErrorStatus> {
        syscalls::io::fsize(self.ri())
    }

    pub const fn ri(&self) -> Ri {
        self.ri as Ri
    }
}

const _: () = assert!(size_of::<FD>() == size_of::<c_int>());

impl Metadata {
    pub fn seek(&mut self, wrench: SeekPosition) {
        match wrench {
            SeekPosition::Start(s) => self.offset = s as isize,
            SeekPosition::End(e) => self.offset = -((e + 1) as isize),
            SeekPosition::Current(c) => {
                if self.offset >= 0 || self.offset < -1 {
                    self.offset = self.offset.saturating_add(c);
                }
            }
        }
    }

    pub fn add(&mut self, count: usize) {
        if self.offset >= 0 {
            self.offset += count as isize;
        }
    }
    pub fn tell(&self, fd: FD) -> Result<usize, ErrorStatus> {
        if self.offset >= 0 {
            Ok(self.offset as usize)
        } else {
            let size = fd.size()?;
            Ok(size.saturating_add_signed(self.offset + 1))
        }
    }
}
static FAKE_FD_META: Mutex<HashMap<u16, Metadata, FxBuildHasher>> =
    Mutex::new(HashMap::with_hasher(FxBuildHasher));
static NEXT_FD_KEY: AtomicU16 = AtomicU16::new(0);

#[derive(Debug)]
#[repr(C)]
pub struct OwnedFD(FD);

impl OwnedFD {
    fn from_resource(resource: Resource, offset: isize) -> Self {
        let ri = resource.ri();
        assert!(
            ri <= u16::MAX as u32,
            "FIXME: OwnedFD cannot handle Resources bigger than u16::MAX"
        );
        let key = NEXT_FD_KEY.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

        FAKE_FD_META.lock().insert(key, Metadata { offset });
        core::mem::forget(resource);
        Self(FD {
            metadata: key,
            ri: ri as u16,
        })
    }
}

impl Drop for OwnedFD {
    fn drop(&mut self) {
        FAKE_FD_META
            .lock()
            .remove(&self.0.metadata)
            .expect("Metadata key was invalid");
        unsafe { drop(Resource::from_raw(self.0.ri())) }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn read(fd: FD, buf: *mut u8, count: usize) -> isize {
    let Some(off) = fd.with_meta(|m| m.offset) else {
        return -1;
    };
    let resource = unsafe { Resource::from_raw(fd.ri()) };

    let slice = unsafe { core::slice::from_raw_parts_mut(buf, count) };

    let ret = unsafe { resource.read(off, slice) };

    core::mem::forget(resource);

    let n = try_errno!(ret, -1);
    fd.with_meta(|m| m.add(n)).unwrap();
    n as isize
}

#[unsafe(no_mangle)]
pub extern "C" fn write(fd: FD, buf: *const u8, count: usize) -> isize {
    let Some(off) = fd.with_meta(|m| m.offset) else {
        return -1;
    };
    let resource = unsafe { Resource::from_raw(fd.ri()) };

    let slice = unsafe { core::slice::from_raw_parts(buf, count) };

    let ret = unsafe { resource.write(off, slice) };

    core::mem::forget(resource);

    let n = try_errno!(ret, -1);
    fd.with_meta(|m| m.add(n)).unwrap();
    n as isize
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

    let mut meta = FAKE_FD_META.lock();

    let Some(m) = meta.get_mut(&fd.metadata) else {
        return -1;
    };

    m.seek(pos);
    try_errno!(m.tell(fd), -1) as isize
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
    let fd = OwnedFD::from_resource(res, 0);

    let ret: c_int = unsafe { core::mem::transmute(fd.0) };
    core::mem::forget(fd);

    ret
}
#[unsafe(no_mangle)]
pub extern "C" fn open(path: *const c_char, flags: c_int, _mode: c_int) -> c_int {
    let cpath = unsafe { CStr::from_ptr(path) };
    let Ok(path) = cpath.to_str() else {
        set_error(ErrorStatus::InvalidPath);
        return -1;
    };

    let mut opts = OpenOptions::from_bits(0);
    let mut off = 0;
    if flags & O_APPEND != 0 {
        off = -1;
    }

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
    let fd = OwnedFD::from_resource(res, off);

    let ret: c_int = unsafe { core::mem::transmute(fd.0) };
    core::mem::forget(fd);

    ret
}

#[unsafe(no_mangle)]
pub extern "C" fn close(fd: FD) -> c_int {
    FAKE_FD_META.lock().remove(&fd.metadata);

    let resource = unsafe { Resource::from_raw(fd.ri()) };

    match resource.destroy() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dup(fd: FD) -> c_int {
    let resource = unsafe { Resource::from_raw(fd.ri()) };
    let clone = try_errno!(resource.clone(), -1);
    core::mem::forget(resource);
    let new_fd = FD {
        ri: clone
            .ri()
            .try_into()
            .expect("Resource ID too big to fit in a Unix FD"),
        ..fd
    };

    core::mem::forget(clone);
    unsafe { core::mem::transmute(new_fd) }
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
