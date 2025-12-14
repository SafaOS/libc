use core::ffi::{c_char, c_int};
use core::{ptr, slice};

#[no_mangle]
pub extern "C" fn strlen(cstr: *const c_char) -> usize {
    let mut p = cstr;
    let mut len = 0;
    unsafe {
        while *p != 0 {
            p = p.add(1);
            len += 1;
        }
    }
    len
}

#[no_mangle]
pub extern "C" fn strnlen(cstr: *const c_char, maxlen: usize) -> usize {
    let mut p = cstr;
    let mut len = 0;
    unsafe {
        while len < maxlen && *p != 0 {
            p = p.add(1);
            len += 1;
        }
    }
    len
}

#[no_mangle]
pub extern "C" fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8 {
    unsafe {
        let len = strlen(src as *const c_char);
        ptr::copy_nonoverlapping(src, dest, len + 1);
    }
    dest
}

#[no_mangle]
pub extern "C" fn strncpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        let len = strnlen(src as *const c_char, n);
        ptr::copy_nonoverlapping(src, dest, len);
        if len < n {
            *dest.add(len) = 0;
        }
    }
    dest
}

#[no_mangle]
pub extern "C" fn stpcpy(dest: *mut u8, src: *const u8) -> *mut u8 {
    unsafe {
        let len = strlen(src as *const c_char);
        ptr::copy_nonoverlapping(src, dest, len + 1);
        dest.add(len)
    }
}

#[no_mangle]
pub extern "C" fn strspn(s: *const u8, accept: *const u8) -> usize {
    unsafe {
        let accept_len = strlen(accept as *const c_char);
        let accept = slice::from_raw_parts(accept, accept_len);

        let mut i = 0;
        let mut p = s;

        while *p != 0 {
            if accept.contains(&*p) {
                i += 1;
                p = p.add(1);
            } else {
                break;
            }
        }
        i
    }
}

#[no_mangle]
pub extern "C" fn strcspn(s: *const u8, reject: *const u8) -> usize {
    unsafe {
        let reject_len = strlen(reject as *const c_char);
        let reject = slice::from_raw_parts(reject, reject_len);

        let mut i = 0;
        let mut p = s;

        while *p != 0 {
            if reject.contains(&*p) {
                break;
            }
            i += 1;
            p = p.add(1);
        }
        i
    }
}

#[no_mangle]
pub extern "C" fn memchr(p: *const u8, c: c_int, n: usize) -> *const u8 {
    unsafe {
        let byte = c as u8;
        for i in 0..n {
            if *p.add(i) == byte {
                return p.add(i);
            }
        }
        ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn strchr(s: *const u8, c: c_int) -> *const u8 {
    unsafe {
        let byte = c as u8;
        let mut p = s;
        while *p != 0 {
            if *p == byte {
                return p;
            }
            p = p.add(1);
        }
        ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn strrchr(s: *const u8, c: c_int) -> *const u8 {
    unsafe {
        let byte = c as u8;
        let mut last = ptr::null();
        let mut p = s;

        while *p != 0 {
            if *p == byte {
                last = p;
            }
            p = p.add(1);
        }
        last
    }
}

#[no_mangle]
pub extern "C" fn strstr(haystack: *const u8, needle: *const u8) -> *const u8 {
    unsafe {
        let hlen = strlen(haystack as *const c_char);
        let nlen = strlen(needle as *const c_char);

        if nlen == 0 {
            return haystack;
        }

        let h = slice::from_raw_parts(haystack, hlen);
        let n = slice::from_raw_parts(needle, nlen);

        for i in 0..=(hlen - nlen) {
            if &h[i..i + nlen] == n {
                return haystack.add(i);
            }
        }
        ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn strcat(dest: *mut u8, src: *const u8) -> *mut u8 {
    unsafe {
        let dlen = strlen(dest as *const c_char);
        let slen = strlen(src as *const c_char);
        ptr::copy_nonoverlapping(src, dest.add(dlen), slen + 1);
    }
    dest
}

#[no_mangle]
pub extern "C" fn strcmp(s1: *const u8, s2: *const u8) -> c_int {
    unsafe {
        let mut p1 = s1;
        let mut p2 = s2;

        while *p1 == *p2 {
            if *p1 == 0 {
                return 0;
            }
            p1 = p1.add(1);
            p2 = p2.add(1);
        }
        (*p1 as c_int) - (*p2 as c_int)
    }
}

#[no_mangle]
pub extern "C" fn strncmp(s1: *const u8, s2: *const u8, n: usize) -> c_int {
    unsafe {
        for i in 0..n {
            let a = *s1.add(i);
            let b = *s2.add(i);
            if a != b {
                return (a as c_int) - (b as c_int);
            }
            if a == 0 {
                return 0;
            }
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn memset(
    ptr: *mut core::ffi::c_void,
    c: c_int,
    n: usize,
) -> *mut core::ffi::c_void {
    unsafe {
        let p = ptr as *mut u8;
        core::ptr::write_bytes(p, c as u8, n);
    }
    ptr
}

#[no_mangle]
pub extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        ptr::copy(src, dest, n);
    }
    dest
}

#[no_mangle]
pub extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> c_int {
    unsafe {
        for i in 0..n {
            let a = *s1.add(i);
            let b = *s2.add(i);
            if a != b {
                return (a as c_int) - (b as c_int);
            }
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn strpbrk(s: *const u8, accept: *const u8) -> *mut u8 {
    unsafe {
        let alen = strlen(accept as *const c_char);
        let a = slice::from_raw_parts(accept, alen);

        let mut p = s;
        while *p != 0 {
            if a.contains(&*p) {
                return p as *mut u8;
            }
            p = p.add(1);
        }
        ptr::null_mut()
    }
}
