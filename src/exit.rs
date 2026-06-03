use core::ffi::c_int;

use alloc::vec::Vec;
use safa_api::{sync::locks::Mutex, syscalls};

static DESTRUCTORS: Mutex<Vec<extern "C" fn()>> = Mutex::new(Vec::new());

#[unsafe(no_mangle)]
pub extern "C" fn atexit(f: extern "C" fn()) -> c_int {
    DESTRUCTORS.lock().push(f);
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn exit(code: c_int) -> ! {
    let destructors = DESTRUCTORS.lock();

    for dest in &*destructors {
        dest();
    }

    drop(destructors);
    _exit(code)
}

#[unsafe(no_mangle)]
pub extern "C" fn _exit(code: c_int) -> ! {
    syscalls::process::exit(code as isize as usize)
}
