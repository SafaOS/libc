use core::cell::UnsafeCell;

use safa_api::errors::ErrorStatus;

static ERRNO_RAW: UnsafeCell<Option<ErrorStatus>> = UnsafeCell::new(None);

/// Sets system errorno to `status`.
pub fn set_error(status: ErrorStatus) {
    unsafe {
        *ERRNO_RAW.get() = Some(status);
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
