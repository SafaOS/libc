use core::{
    cmp::Ordering,
    ffi::{c_char, c_int},
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int {
    unsafe { strncasecmp(a, b, usize::MAX) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncasecmp(a: *const c_char, b: *const c_char, n: usize) -> c_int {
    let mut a_sum = 0;
    let mut b_sum = 0;

    for i in 0..n {
        let at_a = unsafe { a.add(i).read() };
        let at_b = unsafe { b.add(i).read() };

        let val_a = (at_a as u8).to_ascii_lowercase();
        let val_b = (at_b as u8).to_ascii_lowercase();
        a_sum += val_a as usize;
        b_sum += val_b as usize;

        if at_a == 0 || at_b == 0 {
            break;
        }
    }

    match a_sum.cmp(&b_sum) {
        Ordering::Equal => 0,
        Ordering::Less => -1,
        Ordering::Greater => 1,
    }
}
