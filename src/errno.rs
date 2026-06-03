use core::{cell::UnsafeCell, ffi::c_int};

use safa_api::errors::ErrorStatus;

struct ErrorCell(UnsafeCell<u32>);
unsafe impl Sync for ErrorCell {}
unsafe impl Send for ErrorCell {}

#[thread_local]
#[unsafe(no_mangle)]
static errno: ErrorCell = ErrorCell(UnsafeCell::new(0));

#[unsafe(no_mangle)]
pub extern "C" fn __errno_location() -> *mut c_int {
    errno.0.get().cast()
}

/// Sets system errorno to `status`.
pub fn set_error(status: ErrorStatus) {
    unsafe {
        *errno.0.get() = status as u32;
    }
}

/// Similar to `?` syntax but on error also sets errno and returns a given error value.
#[macro_export]
macro_rules! try_errno {
    ($expr:expr, $err:expr) => {{
        match $expr {
            Ok(o) => o,
            Err(e) => {
                $crate::errno::set_error(e);
                return $err;
            }
        }
    }};
}
