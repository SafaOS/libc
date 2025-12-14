#![no_std]
pub mod errno;
pub mod file;
pub mod math;
pub mod setjmp;
pub mod stdio;
pub mod stdlib;
pub mod string;

use safa_api::abi::process::AbiStructures;
use safa_api::ffi::{slice::Slice, str::Str};
use safa_api::syscalls;

unsafe extern "C" {
    fn main() -> isize;
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
        safa_api::process::init::sysapi_init(args, env, *task_abi_structures);

        let results = main();

        syscalls::process::exit(results as usize);
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
