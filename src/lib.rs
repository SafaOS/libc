#![no_std]
#![feature(c_variadic)]

pub mod dirent;
pub mod errno;
pub mod file;
pub mod format;
pub mod math;
pub mod setjmp;
pub mod stdio;
pub mod stdlib;
pub mod string;

pub extern crate alloc;

use core::cell::UnsafeCell;
use core::ffi::c_char;
use core::ops::{Deref, DerefMut};

use safa_api::abi::process::AbiStructures;
use safa_api::ffi::{slice::Slice, str::Str};
use safa_api::process::stdio::{systry_get_stderr, systry_get_stdin, systry_get_stdout};

use crate::file::File;
use crate::stdio::{STDERR_RAW, STDIN_RAW, STDOUT_RAW, stderr, stdin, stdout};

unsafe extern "C" {
    fn main(argc: i32, argv: *const *mut c_char) -> i32;
}

extern "C" fn _libc_init(argc: i32, argv: *const *const u8) -> i32 {
    if let Some(ri) = systry_get_stdout().into() {
        unsafe {
            let r = (*STDOUT_RAW.get()).write(File::from_res(
                ri,
                file::BufferingOption::LineBuffered,
                false,
            ));
            *stdout.0.get() = r;
        }
    };

    if let Some(ri) = systry_get_stderr().into() {
        unsafe {
            let r =
                (*STDERR_RAW.get()).write(File::from_res(ri, file::BufferingOption::None, false));
            *stderr.0.get() = r;
        }
    };

    if let Some(ri) = systry_get_stdin().into() {
        unsafe {
            let r = (*STDIN_RAW.get()).write(File::from_res(
                ri,
                file::BufferingOption::Buffered,
                false,
            ));
            *stdin.0.get() = r;
        }
    };
    return unsafe { main(argc, argv.cast()) };
}

#[unsafe(no_mangle)]
unsafe extern "C" fn _start_inner(
    argc: usize,
    argv: *mut Str,
    envc: usize,
    envp: *mut Slice<u8>,
    task_abi_structures: *const AbiStructures,
) -> ! {
    unsafe {
        let args = Slice::from_raw_parts(argv, argc);
        let env = Slice::from_raw_parts(envp, envc);
        safa_api::process::init::_c_api_init(args, env, task_abi_structures, _libc_init)
    }
}

#[unsafe(no_mangle)]
#[allow(unused)]
#[unsafe(naked)]
pub extern "C" fn _start(
    argc: usize,
    argv: *mut Str,
    envc: usize,
    envp: *mut Slice<u8>,
    task_abi_structures: *const AbiStructures,
) {
    unsafe {
        #[cfg(target_arch = "aarch64")]
        core::arch::naked_asm!(
            "
            mov fp, #0
            sub sp, sp, #16
            stp xzr, xzr, [sp]
            bl _start_inner
            "
        );
        #[cfg(target_arch = "x86_64")]
        core::arch::naked_asm!(
            "
            and rsp, ~0xf
            push rbp
            push rbp
            call _start_inner
            ud2
        ",
        );
    };
}

#[derive(Debug)]
pub struct SyncUnsafeCell<T> {
    pub inner: UnsafeCell<T>,
}

impl<T> SyncUnsafeCell<T> {
    pub const fn new(v: T) -> Self {
        Self {
            inner: UnsafeCell::new(v),
        }
    }
}

unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}
impl<T> Deref for SyncUnsafeCell<T> {
    type Target = UnsafeCell<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for SyncUnsafeCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
