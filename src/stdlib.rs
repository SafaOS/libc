use core::ffi::c_double;
use core::ptr::NonNull;
use core::{
    ffi::{c_char, c_int, c_void},
    ptr, slice,
};

use safa_api::abi::process::SpawnFlags;
use safa_api::alloc as api_alloc;
use safa_api::process::env;
use safa_api::syscalls;

extern crate alloc;

use crate::string::strlen;
use crate::try_errno;

unsafe fn cstr_to_bytes<'a>(p: *const c_char) -> &'a [u8] {
    if p.is_null() {
        return &[];
    }
    let len = strlen(p);
    unsafe { slice::from_raw_parts(p as *const u8, len) }
}

unsafe fn cstr_to_str<'a>(p: *const c_char) -> Option<&'a str> {
    let bytes = unsafe { cstr_to_bytes(p) };
    core::str::from_utf8(bytes).ok()
}

#[unsafe(no_mangle)]
pub extern "C" fn abs(x: i32) -> u32 {
    x.unsigned_abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn exit(code: c_int) -> ! {
    syscalls::process::exit(code as isize as usize)
}

#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
    exit(-1)
}

const ALIGNMENT: usize = align_of::<usize>() * 2;

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
    if size == 0 {
        return ptr::null_mut();
    }

    match api_alloc::GLOBAL_SYSTEM_ALLOCATOR.allocate(size, ALIGNMENT) {
        Some(nonnull_slice) => nonnull_slice.as_ptr() as *mut c_void,
        None => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut c_void) {
    let non_null = NonNull::new(ptr);
    if let Some(p) = non_null {
        unsafe { api_alloc::GLOBAL_SYSTEM_ALLOCATOR.deallocate(p.cast()) }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn calloc(nmemb: usize, size: usize) -> *mut c_void {
    match nmemb.checked_mul(size) {
        Some(total) => {
            if total == 0 {
                return ptr::null_mut();
            }

            unsafe {
                use core::alloc::{GlobalAlloc, Layout};
                let layout = match Layout::from_size_align(total, ALIGNMENT) {
                    Ok(l) => l,
                    Err(_) => return ptr::null_mut(),
                };

                let p = api_alloc::GLOBAL_SYSTEM_ALLOCATOR.alloc_zeroed(layout);
                if p.is_null() {
                    ptr::null_mut()
                } else {
                    p as *mut c_void
                }
            }
        }
        None => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    if ptr.is_null() {
        return malloc(new_size);
    }
    if new_size == 0 {
        free(ptr);
        return ptr::null_mut();
    }

    unsafe {
        use core::alloc::{GlobalAlloc, Layout};
        // TODO: we cannot rely on size here, I use usize::MAX so GlobalAlloc would copy new_size bytes.
        let layout = match Layout::from_size_align(usize::MAX, ALIGNMENT) {
            Ok(l) => l,
            Err(_) => return ptr::null_mut(),
        };

        let newp = api_alloc::GLOBAL_SYSTEM_ALLOCATOR.realloc(ptr as *mut u8, layout, new_size);
        if newp.is_null() {
            ptr::null_mut()
        } else {
            newp as *mut c_void
        }
    }
}

//
// -- environment + system/process
//
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getenv(name: *const c_char) -> *const c_char {
    if name.is_null() {
        return ptr::null();
    }
    unsafe {
        let bytes = cstr_to_bytes(name);

        if let Some(value_bytes) = env::env_get(bytes) {
            return value_bytes.as_ptr() as *const c_char;
        }
    }
    ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setenv(
    name: *const c_char,
    value: *const c_char,
    overwrite: i32,
) -> c_int {
    if name.is_null() || value.is_null() {
        return -1;
    }
    unsafe {
        let name_bytes = cstr_to_bytes(name);
        let value_bytes = cstr_to_bytes(value);

        // NOTE: The Zig code had an odd overwrite check; preserve behaviour:
        if overwrite != 0 {
            if env::env_get(name_bytes).is_some() {
                return 0;
            }
        }

        env::env_set(name_bytes, value_bytes);
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn unsetenv(name: *const c_char) -> c_int {
    if name.is_null() {
        return -1;
    }
    unsafe {
        let name_bytes = cstr_to_bytes(name);
        env::env_remove(name_bytes);
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn system(command_raw: *const c_char) -> c_int {
    unsafe {
        let shell_opt = env::env_get(b"SHELL");
        if command_raw.is_null() {
            return if shell_opt.is_some() { 1 } else { 0 };
        }

        let shell = match shell_opt.as_ref() {
            Some(b) => match core::str::from_utf8(&*b) {
                Ok(s) => s,
                Err(_) => return -1,
            },
            None => return -1, // Zig panicked here if SHELL unset; we return -1
        };

        let cmd_bytes = cstr_to_bytes(command_raw as *const c_char);
        let cmd = match core::str::from_utf8(cmd_bytes) {
            Ok(s) => s,
            Err(_) => return -1,
        };

        use alloc::vec;
        use alloc::vec::Vec;

        let args: Vec<&str> = vec![shell, "-c", cmd];

        // spawn + wait: safa-api has a process module for high-level process ops. Use it.
        // The exact function name / return type may vary by safa-api version; adjust if necessary.
        let pid = try_errno!(
            syscalls::process::spawn(
                Some(shell),
                shell,
                args,
                SpawnFlags::CLONE_CWD | SpawnFlags::CLONE_RESOURCES,
                safa_api::abi::process::RawContextPriority::Default,
                None,
                None,
                None,
                None,
            ),
            -1
        );

        let status = try_errno!(syscalls::process::wait(pid), -1);
        status as c_int
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn atoi(c_str: *const c_char) -> c_int {
    if c_str.is_null() {
        return 0;
    }
    unsafe {
        let Some(str) = cstr_to_str(c_str) else {
            return -1;
        };

        match str.parse::<c_int>() {
            Ok(v) => v,
            Err(_) => 0,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn atof(c_str: *const c_char) -> c_double {
    if c_str.is_null() {
        return 0.;
    }
    unsafe {
        let Some(str) = cstr_to_str(c_str) else {
            return 0.;
        };

        match str.parse::<c_double>() {
            Ok(v) => v,
            Err(_) => 0.,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn strtod(ptr: *const c_char, endptr: *mut *const c_char) -> f64 {
    if ptr.is_null() {
        return 0.0;
    }

    unsafe {
        let Some(bytes) = cstr_to_str(ptr) else {
            return 0.0;
        };
        let slice = bytes.trim_ascii();

        match slice.parse::<f64>() {
            Ok(val) => {
                // TODO: a bit of a stub
                if !endptr.is_null() {
                    *endptr = slice.as_ptr().add(slice.len()).cast();
                }
                val
            }
            Err(_) => {
                if !endptr.is_null() {
                    *endptr = ptr as *mut c_char;
                }
                0.0
            }
        }
    }
}
