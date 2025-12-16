use core::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int {
    unsafe { strncasecmp(a, b, usize::MAX) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncasecmp(a: *const c_char, b: *const c_char, n: usize) -> c_int {
    for i in 0..n {
        let at_a = unsafe { a.add(i).read() };
        let at_b = unsafe { b.add(i).read() };

        let val_a = (at_a as u8).to_ascii_lowercase();
        let val_b = (at_b as u8).to_ascii_lowercase();

        if val_a != val_b {
            return val_a.cmp(&val_b) as c_int;
        }

        if at_a == 0 || at_b == 0 {
            break;
        }
    }

    0
}
