use core::ffi::{CStr, c_double, c_long, c_longlong, c_uint, c_ulong, c_ulonglong};
use core::num::IntErrorKind;
use core::ptr::NonNull;
use core::{
    ffi::{c_char, c_int, c_void},
    ptr,
};

use rand_pcg::Pcg32;
use rand_pcg::rand_core::{Rng, SeedableRng};
use safa_api::abi::process::SpawnFlags;
use safa_api::alloc as api_alloc;
use safa_api::errors::ErrorStatus;
use safa_api::process::env;
use safa_api::process::stdio::{systry_get_stderr, systry_get_stdin, systry_get_stdout};
use safa_api::syscalls;

extern crate alloc;

use crate::errno::set_error;
use crate::{SyncUnsafeCell, try_errno};

unsafe fn cstr_to_bytes<'a>(p: *const c_char) -> &'a [u8] {
    if p.is_null() {
        return &[];
    }

    unsafe { CStr::from_ptr(p).to_bytes() }
}

unsafe fn cstr_to_str<'a>(p: *const c_char) -> Option<&'a str> {
    unsafe { CStr::from_ptr(p).to_str().ok() }
}

#[unsafe(no_mangle)]
pub extern "C" fn abs(x: c_int) -> c_int {
    x.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn llabs(x: c_longlong) -> c_longlong {
    x.abs()
}

const ALIGNMENT: usize = align_of::<usize>() * 2;

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
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
        Some(total) => unsafe {
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
        },
        None => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    if ptr.is_null() {
        return malloc(new_size);
    }

    unsafe {
        match api_alloc::GLOBAL_SYSTEM_ALLOCATOR.reallocate(
            NonNull::new_unchecked(ptr.cast()),
            new_size,
            ALIGNMENT,
        ) {
            Some(p) => p.as_ptr().cast(),
            None => core::ptr::null_mut(),
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
pub unsafe extern "C" fn putenv(name: *const c_char) -> c_int {
    if name.is_null() {
        return -1;
    }
    unsafe {
        let bytes = cstr_to_bytes(name);
        let mut splt = bytes.splitn(2, |c| *c == b'=');
        let name = splt.next().unwrap_or(b"");
        let value = splt.next().unwrap_or(b"");
        env::env_set(name, value);
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
                systry_get_stdin().into(),
                systry_get_stdout().into(),
                systry_get_stderr().into(),
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
        let bytes = cstr_to_bytes(ptr);
        let slice = bytes.trim_ascii();
        let mut dotted = false;
        let mut e = false;
        let mut e_begin = false;

        let Some(slice) = first_valid_slice(
            &[],
            &[b"+", b"-"],
            |c| {
                if c.is_ascii_digit() {
                    true
                } else if c == b'.' && !dotted {
                    dotted = true;
                    true
                } else if (c == b'e' || c == b'E') && !e {
                    dotted = false;
                    e_begin = true;
                    e = true;
                    true
                } else if (c == b'-' || c == b'+') && e_begin {
                    e_begin = false;
                    true
                } else {
                    false
                }
            },
            slice,
        ) else {
            if !endptr.is_null() {
                *endptr = ptr as *mut c_char;
            }
            return 0.0;
        };

        match str::from_utf8(slice)
            .expect("Failed to validate str likely a bug")
            .parse::<f64>()
        {
            Ok(val) => {
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

fn first_valid_slice<'a>(
    trim_prefix: &[&[u8]],
    valid_prefix: &[&[u8]],
    mut is_valid: impl FnMut(u8) -> bool,
    str: &'a [u8],
) -> Option<&'a [u8]> {
    let mut str = str.trim_ascii();
    // FIXME: Not so correct impl
    for p in trim_prefix {
        str = str.trim_prefix(*p);
    }
    let mut check_pos = 0;
    while check_pos < str.len() {
        // FIXME: also not so correct
        if !is_valid(str[check_pos]) && !valid_prefix.contains(&&str[..check_pos]) {
            break;
        }
        check_pos += 1;
    }
    (check_pos != 0).then(|| &str[..check_pos])
}

macro_rules! strtox {
    ($x:ty, $singed:literal, $ptr:ident,$endptr:ident,$base:ident) => {{
        if $ptr.is_null() || $base == 1 || $base > 36 {
            return 0;
        }

        unsafe {
            let str = cstr_to_bytes($ptr);
            let str = str.trim_ascii();
            if $base == 0 {
                if str.starts_with(b"0x") || str.starts_with(b"0X") {
                    $base = 16;
                } else if str.starts_with(b"0") {
                    $base = 8;
                } else {
                    $base = 10;
                }
            }

            let Some(slice) = first_valid_slice(
                &[b"0x", b"0X"],
                &[b"-", b"+"],
                |c| c >= b'0' && (((c - b'0') as i32) < ($base - 1)),
                str,
            ) else {
                if !$endptr.is_null() {
                    *$endptr = $ptr as *mut c_char;
                }
                return 0;
            };

            match <$x>::from_str_radix(
                core::str::from_utf8(slice)
                    .expect("Failed to validate a number string likely a bug"),
                $base as u32,
            ) {
                Ok(val) => {
                    // TODO: a bit of a stub
                    if !$endptr.is_null() {
                        *$endptr = str.as_ptr().add(str.len()).cast();
                    }
                    val
                }
                Err(e) => match e.kind() {
                    IntErrorKind::PosOverflow => {
                        if !$endptr.is_null() {
                            *$endptr = str.as_ptr().add(str.len()).cast();
                        }
                        set_error(ErrorStatus::StrTooLong);
                        <$x>::MAX
                    }
                    IntErrorKind::NegOverflow => {
                        if !$endptr.is_null() {
                            *$endptr = str.as_ptr().add(str.len()).cast();
                        }
                        set_error(ErrorStatus::StrTooLong);
                        <$x>::MIN
                    }
                    IntErrorKind::InvalidDigit => {
                        unreachable!("Number string validation is bugged")
                    }
                    _ => {
                        if !$endptr.is_null() {
                            *$endptr = $ptr as *mut c_char;
                        }
                        0
                    }
                },
            }
        }
    }};
}

#[unsafe(no_mangle)]
pub extern "C" fn strtol(
    ptr: *const c_char,
    endptr: *mut *const c_char,
    mut base: c_int,
) -> c_long {
    strtox!(c_long, true, ptr, endptr, base)
}

#[unsafe(no_mangle)]
pub extern "C" fn strtoll(
    ptr: *const c_char,
    endptr: *mut *const c_char,
    mut base: c_int,
) -> c_longlong {
    strtox!(c_longlong, true, ptr, endptr, base)
}

#[unsafe(no_mangle)]
pub extern "C" fn strtoul(
    ptr: *const c_char,
    endptr: *mut *const c_char,
    mut base: c_int,
) -> c_ulong {
    strtox!(c_ulong, false, ptr, endptr, base)
}

#[unsafe(no_mangle)]
pub extern "C" fn strtoull(
    ptr: *const c_char,
    endptr: *mut *const c_char,
    mut base: c_int,
) -> c_ulonglong {
    strtox!(c_ulonglong, false, ptr, endptr, base)
}

static RNG: SyncUnsafeCell<Option<Pcg32>> = SyncUnsafeCell::new(None);
#[unsafe(no_mangle)]
pub extern "C" fn srand(seed: c_uint) {
    if seed == 1 {
        unsafe {
            (*RNG.get()) = Some(Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7));
        }
        return;
    }
    unsafe {
        (*RNG.get()) = Some(Pcg32::seed_from_u64(seed as u64));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rand() -> c_int {
    let rng = unsafe { &mut (*RNG.get()) }
        .get_or_insert_with(|| Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7));
    rng.next_u32() as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bsearch(
    key: *const c_void,
    base: *const c_void,
    count: usize,
    size: usize,
    compare: extern "C" fn(*const c_void, *const c_void) -> c_int,
) -> *const c_void {
    if count == 0 || size == 0 {
        return core::ptr::null();
    }

    let mut low = 0usize;
    let mut high = count;

    while low < high {
        let mid = low + (high - low) / 2;

        // SAFETY: mid is always in bounds [0, count), and base points to
        // count * size valid bytes (caller's responsibility per C contract).
        let element = unsafe { (base as *const u8).add(mid * size) as *const c_void };

        match compare(key, element) {
            0 => return element as *mut c_void,
            n if n < 0 => high = mid,
            _ => low = mid + 1,
        }
    }

    core::ptr::null_mut()
}

// My qsort was really slow and I'm lazy, so the implementation below was generated by claude.
use core::cmp::Ordering;

type CompFn = extern "C" fn(*const u8, *const u8) -> c_int;

const INSERTION_THRESHOLD: usize = 16;

#[inline(always)]
unsafe fn elem_ptr(base: *mut u8, idx: usize, size: usize) -> *mut u8 {
    unsafe { base.add(idx * size) }
}

#[inline(always)]
unsafe fn swap_elems(base: *mut u8, i: usize, j: usize, size: usize) {
    if i == j {
        return;
    }
    unsafe {
        let pi = elem_ptr(base, i, size);
        let pj = elem_ptr(base, j, size);
        // In-place, no heap allocation.
        core::ptr::swap_nonoverlapping(pi, pj, size);
    }
}

#[inline(always)]
unsafe fn compare(base: *mut u8, i: usize, j: usize, size: usize, comp: CompFn) -> Ordering {
    unsafe {
        let pi = elem_ptr(base, i, size) as *const u8;
        let pj = elem_ptr(base, j, size) as *const u8;
        match comp(pi, pj) {
            r if r < 0 => Ordering::Less,
            0 => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
}

unsafe fn insertion_sort(base: *mut u8, lo: usize, hi: usize, size: usize, comp: CompFn) {
    let mut i = lo + 1;
    while i <= hi {
        let mut j = i;
        while j > lo && unsafe { compare(base, j, j - 1, size, comp) } == Ordering::Less {
            unsafe { swap_elems(base, j, j - 1, size) };
            j -= 1;
        }
        i += 1;
    }
}

#[inline]
unsafe fn median_of_three(
    base: *mut u8,
    a: usize,
    b: usize,
    c: usize,
    size: usize,
    comp: CompFn,
) -> usize {
    unsafe {
        let ab = compare(base, a, b, size, comp);
        let bc = compare(base, b, c, size, comp);
        let ac = compare(base, a, c, size, comp);
        if ab == Ordering::Less {
            if bc == Ordering::Less {
                b
            } else if ac == Ordering::Less {
                c
            } else {
                a
            }
        } else {
            if ac == Ordering::Less {
                a
            } else if bc == Ordering::Less {
                c
            } else {
                b
            }
        }
    }
}

unsafe fn quicksort(base: *mut u8, lo: usize, hi: usize, size: usize, comp: CompFn) {
    if hi <= lo {
        return;
    }
    let len = hi - lo + 1;
    if len <= INSERTION_THRESHOLD {
        unsafe { insertion_sort(base, lo, hi, size, comp) };
        return;
    }

    let mid = lo + len / 2;
    let pivot_idx = unsafe { median_of_three(base, lo, mid, hi, size, comp) };
    unsafe { swap_elems(base, pivot_idx, hi, size) }; // pivot -> end

    // Lomuto partition against the pivot now sitting at `hi`.
    let mut store = lo;
    for k in lo..hi {
        if unsafe { compare(base, k, hi, size, comp) } == Ordering::Less {
            unsafe { swap_elems(base, k, store, size) };
            store += 1;
        }
    }
    unsafe { swap_elems(base, store, hi, size) };

    if store > lo {
        unsafe { quicksort(base, lo, store - 1, size, comp) };
    }
    if store < hi {
        unsafe { quicksort(base, store + 1, hi, size, comp) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn qsort(base_ptr: *mut u8, count: usize, size: usize, comp: CompFn) {
    if base_ptr.is_null() || count <= 1 || size == 0 {
        return;
    }
    unsafe { quicksort(base_ptr, 0, count - 1, size, comp) };
}
