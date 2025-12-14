use core::{
    ffi::{CStr, c_char, c_int, c_void},
    ptr::null_mut,
};

use safa_api::{
    abi::fs::OpenOptions,
    errors::ErrorStatus,
    syscalls::{fs, io, resources, types::Ri},
};

use crate::{
    file::{File, SeekPosition},
    try_errno,
};

extern crate alloc;
use alloc::boxed::Box;

// ==========================
// File management
// ==========================

fn fopen_inner(filename: *const c_char, mode: *const c_char) -> File {
    let cstr_path = unsafe { CStr::from_ptr(filename) };
    let cstr_mode = unsafe { CStr::from_ptr(mode) };

    let path = try_errno!(
        cstr_path.to_str().map_err(|_| ErrorStatus::InvalidStr),
        null_mut()
    );

    let options = OpenOptions::from_bits(0);
    let append = false;
    let create_new = false;

    for b in cstr_mode.to_bytes() {
        match b {
            'w' => options = options | OpenOptions::WRITE | OpenOptions::CREATE_FILE,
            'a' => {
                options = options | OpenOptions::CREATE_FILE;
                append = true
            }
            'r' => options = options | OpenOptions::READ,
            'x' => create_new = true,
            _ => continue,
        }
    }

    if create_new
        && File::open(path, OpenOptions::from_bits(0)) != Err(ErrorStatus::NoSuchAFileOrDirectory)
    // File exists
    {
        return null_mut();
    }

    let f = try_errno!(File::open(path, options), null_mut());
    if append {
        f.seek(SeekPosition::End(0));
    }
    f
}

#[no_mangle]
pub unsafe extern "C" fn fopen(filename: *const c_char, mode: *const c_char) -> *mut File {
    Box::leak(Box::new(fopen_inner(filename, mode)))
}

#[no_mangle]
pub unsafe extern "C" fn fclose(file: *mut File) -> c_int {
    let boxed = unsafe { Box::from_raw(file) };
    try_errno!(boxed.close(), -1);
    0
}

#[no_mangle]
pub extern "C" fn freopen(
    filename: *const c_char,
    mode: *const c_char,
    file: *mut File,
) -> *mut File {
    let mut old = unsafe { Box::from_raw(file) };
    unsafe { try_errno!(old.close_ref(), null_mut()) };
    *old = fopen_inner(filename, mode);

    Box::leak(old)
}

#[no_mangle]
pub extern "C" fn remove(path: *const c_char) -> c_int {
    let cstr_path = unsafe { CStr::from_ptr(filename) };
    let path = try_errno!(
        cstr_path.to_str().map_err(|_| ErrorStatus::InvalidStr),
        null_mut()
    );

    try_errno!(fs::remove_path(path), -1);
    0
}

#[no_mangle]
pub extern "C" fn rename(old: *const c_char, new: *const c_char) -> c_int {
    let _ = (old, new);
    todo!("rename")
}

// ==========================
// Reading / writing
// ==========================

#[no_mangle]
pub unsafe extern "C" fn fread(
    ptr: *mut c_void,
    size: usize,
    count: usize,
    stream: *mut File,
) -> usize {
    let stream = unsafe { &mut *stream };
    let buf = unsafe { core::slice::from_raw_parts_mut(ptr.cast::<u8>(), size * count) };
    try_errno!(stream.read(buf), core::usize::MAX)
}

#[no_mangle]
pub extern "C" fn fwrite(
    ptr: *const c_void,
    size: usize,
    count: usize,
    stream: *mut File,
) -> usize {
    let _ = (ptr, size, count, stream);
    todo!("fwrite")
}

#[no_mangle]
pub extern "C" fn fgetc(stream: *mut File) -> c_int {
    let _ = stream;
    todo!("fgetc")
}

#[no_mangle]
pub extern "C" fn getc(stream: *mut File) -> c_int {
    let _ = stream;
    todo!("getc")
}

#[no_mangle]
pub extern "C" fn getchar() -> c_int {
    todo!("getchar")
}

#[no_mangle]
pub extern "C" fn ungetc(c: c_int, stream: *mut File) -> c_int {
    let _ = (c, stream);
    todo!("ungetc")
}

#[no_mangle]
pub extern "C" fn fputc(c: c_int, stream: *mut File) -> c_int {
    let _ = (c, stream);
    todo!("fputc")
}

#[no_mangle]
pub extern "C" fn fputs(s: *const c_char, stream: *mut File) -> c_int {
    let _ = (s, stream);
    todo!("fputs")
}

#[no_mangle]
pub extern "C" fn fgets(s: *mut c_char, size: c_int, stream: *mut File) -> *mut c_char {
    let _ = (s, size, stream);
    todo!("fgets")
}

// ==========================
// Positioning
// ==========================

#[no_mangle]
pub extern "C" fn fseek(stream: *mut File, offset: c_int, whence: c_int) -> c_int {
    let _ = (stream, offset, whence);
    todo!("fseek")
}

#[no_mangle]
pub extern "C" fn ftell(stream: *mut File) -> c_int {
    let _ = stream;
    todo!("ftell")
}

#[no_mangle]
pub extern "C" fn rewind(stream: *mut File) {
    let _ = stream;
    todo!("rewind")
}

// ==========================
// State / errors
// ==========================

#[no_mangle]
pub extern "C" fn feof(stream: *mut File) -> c_int {
    let _ = stream;
    todo!("feof")
}

#[no_mangle]
pub extern "C" fn ferror(stream: *mut File) -> c_int {
    let _ = stream;
    todo!("ferror")
}

#[no_mangle]
pub extern "C" fn clearerr(stream: *mut File) {
    let _ = stream;
    todo!("clearerr")
}

#[no_mangle]
pub extern "C" fn fflush(stream: *mut File) -> c_int {
    let _ = stream;
    todo!("fflush")
}

// ==========================
// Temporary files
// ==========================

#[no_mangle]
pub extern "C" fn tmpfile() -> *mut File {
    todo!("tmpfile")
}

#[no_mangle]
pub extern "C" fn tmpnam(s: *mut c_char) -> *mut c_char {
    let _ = s;
    todo!("tmpnam")
}

// ==========================
// Formatted output (varargs — intentionally unimplemented)
// ==========================

#[no_mangle]
pub extern "C" fn printf(fmt: *const c_char /* ... */) -> c_int {
    let _ = fmt;
    todo!("printf (varargs)")
}

#[no_mangle]
pub extern "C" fn fprintf(stream: *mut File, fmt: *const c_char /* ... */) -> c_int {
    let _ = (stream, fmt);
    todo!("fprintf (varargs)")
}

#[no_mangle]
pub extern "C" fn sprintf(s: *mut c_char, fmt: *const c_char /* ... */) -> c_int {
    let _ = (s, fmt);
    todo!("sprintf (varargs)")
}

#[no_mangle]
pub extern "C" fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char /* ... */) -> c_int {
    let _ = (s, n, fmt);
    todo!("snprintf (varargs)")
}
