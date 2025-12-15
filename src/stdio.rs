use core::{
    ffi::{CStr, VaList, c_char, c_int, c_void},
    mem::MaybeUninit,
    ptr::null_mut,
};

use safa_api::{abi::fs::OpenOptions, errors::ErrorStatus, syscalls::fs};

use crate::parse::{BufReader, CReader};
use crate::{
    SyncUnsafeCell,
    file::{BufferingOption, File, SeekPosition},
    format::BufWriter,
    string::strlen,
    try_errno,
};

extern crate alloc;
use alloc::{boxed::Box, vec::Vec};

#[used]
pub static STDIN_RAW: SyncUnsafeCell<MaybeUninit<File>> =
    SyncUnsafeCell::new(MaybeUninit::uninit());
#[used]
pub static STDOUT_RAW: SyncUnsafeCell<MaybeUninit<File>> =
    SyncUnsafeCell::new(MaybeUninit::uninit());
#[used]
pub static STDERR_RAW: SyncUnsafeCell<MaybeUninit<File>> =
    SyncUnsafeCell::new(MaybeUninit::uninit());

#[derive(Debug)]
pub struct StdIo(pub SyncUnsafeCell<*mut File>);
unsafe impl Send for StdIo {}
unsafe impl Sync for StdIo {}

#[unsafe(no_mangle)]
pub static stdin: StdIo = StdIo(SyncUnsafeCell::new(null_mut()));
#[unsafe(no_mangle)]
pub static stdout: StdIo = StdIo(SyncUnsafeCell::new(null_mut()));
#[unsafe(no_mangle)]
pub static stderr: StdIo = StdIo(SyncUnsafeCell::new(null_mut()));

// ==========================
// File management
// ==========================

fn fopen_inner(filename: *const c_char, mode: *const c_char) -> Option<File> {
    let cstr_path = unsafe { CStr::from_ptr(filename) };
    let cstr_mode = unsafe { CStr::from_ptr(mode) };

    let path = try_errno!(
        cstr_path.to_str().map_err(|_| ErrorStatus::InvalidStr),
        None
    );

    let mut options = OpenOptions::from_bits(0);
    let mut append = false;
    let mut create_new = false;

    for b in cstr_mode.to_bytes() {
        match b {
            b'w' => options = options | OpenOptions::WRITE | OpenOptions::CREATE_FILE,
            b'a' => {
                options = options | OpenOptions::CREATE_FILE;
                append = true
            }
            b'r' => options = options | OpenOptions::READ,
            b'x' => create_new = true,
            _ => continue,
        }
    }

    if create_new
        && File::open(path, OpenOptions::from_bits(0)) != Err(ErrorStatus::NoSuchAFileOrDirectory)
    // File exists
    {
        return None;
    }

    let mut f = try_errno!(File::open(path, options), None);
    if append {
        f.seek(SeekPosition::End(0));
    }
    Some(f)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fopen(filename: *const c_char, mode: *const c_char) -> *mut File {
    match fopen_inner(filename, mode) {
        Some(o) => Box::leak(Box::new(o)),
        None => null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fclose(file: *mut File) -> c_int {
    let boxed = unsafe { Box::from_raw(file) };
    try_errno!(boxed.close(), -1);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn freopen(
    filename: *const c_char,
    mode: *const c_char,
    file: *mut File,
) -> *mut File {
    let mut old = unsafe { Box::from_raw(file) };
    unsafe { try_errno!(old.close_ref(), null_mut()) };
    let Some(new) = fopen_inner(filename, mode) else {
        return null_mut();
    };

    *old = new;

    Box::leak(old)
}

#[unsafe(no_mangle)]
pub extern "C" fn remove(path: *const c_char) -> c_int {
    let cstr_path = unsafe { CStr::from_ptr(path) };
    let path = try_errno!(cstr_path.to_str().map_err(|_| ErrorStatus::InvalidStr), -1);

    try_errno!(fs::remove_path(path), -1);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn rename(old: *const c_char, new: *const c_char) -> c_int {
    let _ = (old, new);
    todo!("rename")
}

// ==========================
// Reading / writing
// ==========================

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fwrite(
    ptr: *const c_void,
    size: usize,
    count: usize,
    stream: *mut File,
) -> usize {
    let stream = unsafe { &mut *stream };
    let buf = unsafe { core::slice::from_raw_parts(ptr.cast::<u8>(), size * count) };
    try_errno!(stream.write(buf), core::usize::MAX)
}

#[unsafe(no_mangle)]
pub extern "C" fn fgetc(stream: *mut File) -> c_int {
    let mut buf = [0u8; 1];
    let stream = unsafe { &mut *stream };
    try_errno!(stream.read(&mut buf), -1);
    buf[0] as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn getc(stream: *mut File) -> c_int {
    fgetc(stream)
}

#[unsafe(no_mangle)]
pub extern "C" fn getchar() -> c_int {
    unsafe { fgetc(*stdin.0.get()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn putchar(c: c_int) -> c_int {
    unsafe { fputc(c, *stdout.0.get()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn ungetc(c: c_int, stream: *mut File) -> c_int {
    let _ = (c, stream);
    todo!("ungetc")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fputc(c: c_int, stream: *mut File) -> c_int {
    let stream = unsafe { &mut *stream };
    let buf = [c as u8];
    loop {
        let r = try_errno!(stream.write(&buf), -1);
        if r == 1 {
            return 0;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fputs(s: *const c_char, stream: *mut File) -> c_int {
    let stream = unsafe { &mut *stream };
    let cstr = unsafe { CStr::from_ptr(s) };
    let bytes = cstr.to_bytes();

    loop {
        let r = try_errno!(stream.write(bytes), -1);
        if r == bytes.len() {
            return 0;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fgets(s: *mut c_char, size: c_int, stream: *mut File) -> *mut c_char {
    let stream = unsafe { &mut *stream };
    let size = size as usize;

    let buf = unsafe { core::slice::from_raw_parts_mut(s.cast::<u8>(), size) };
    let max = size - 1;

    let amount = try_errno!(
        stream.read_bytes_until_or_eof(&mut buf[..max], |c| c == b'\n'),
        null_mut()
    );

    buf[amount] = 0;
    s
}

#[unsafe(no_mangle)]
extern "C" fn fgetline(file: *mut File, len: *mut usize) -> *mut c_char {
    unsafe {
        let mut buf = Vec::new();

        try_errno!(
            (*file).read_bytes_until_or_eof_alloc(&mut buf, usize::MAX, |c| c == b'\n'),
            null_mut()
        );

        *len = buf.len();

        let p = buf.as_mut_ptr();
        core::mem::forget(buf);
        p as *mut c_char
    }
}

#[unsafe(no_mangle)]
// TODO: add Custom buffering
extern "C" fn setvbuf(file: *mut File, custom_buffer: *mut u8, mode: u8, size: usize) -> c_int {
    let Some(mode) = BufferingOption::from_u8(mode) else {
        return -1;
    };

    assert_eq!(
        custom_buffer,
        null_mut(),
        "Custom buffering isn't yet supported"
    );
    unsafe {
        (*file).set_buffering(mode, size);
    }
    return 0;
}

// ==========================
// Positioning
// ==========================

#[unsafe(no_mangle)]
pub extern "C" fn fseek(stream: *mut File, offset: c_int, whence: c_int) -> c_int {
    let pos = match (whence, offset >= 0) {
        (0, true) => SeekPosition::Start(offset as usize),
        (0, false) => SeekPosition::End((-offset) as usize),
        (1, true | false) => SeekPosition::Current(offset as isize),
        (2, true) => SeekPosition::End(offset as usize),
        (2, false) => SeekPosition::Start((-offset) as usize),
        _ => return -1,
    };

    unsafe {
        (*stream).seek(pos);
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ftell(stream: *mut File) -> c_int {
    let stream = unsafe { &*stream };
    stream.offset() as c_int
}

// ==========================
// State / errors
// ==========================

#[unsafe(no_mangle)]
pub extern "C" fn feof(stream: *mut File) -> c_int {
    let stream = unsafe { &*stream };
    stream.is_eof() as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn ferror(stream: *mut File) -> c_int {
    let _ = stream;
    todo!("ferror")
}

#[unsafe(no_mangle)]
pub extern "C" fn clearerr(stream: *mut File) {
    let _ = stream;
    todo!("clearerr")
}

#[unsafe(no_mangle)]
pub extern "C" fn fflush(stream: *mut File) -> c_int {
    let stream = unsafe { &mut *stream };
    try_errno!(stream.flush().map(|()| 0), -1)
}

// ==========================
// Temporary files
// ==========================

#[unsafe(no_mangle)]
pub extern "C" fn tmpfile() -> *mut File {
    todo!("tmpfile")
}

#[unsafe(no_mangle)]
pub extern "C" fn tmpnam(s: *mut c_char) -> *mut c_char {
    let _ = s;
    todo!("tmpnam")
}

// ==========================
// Formatted output (varargs — intentionally unimplemented)
// ==========================

#[unsafe(no_mangle)]
pub unsafe extern "C" fn printf(fmt: *const c_char, mut args: ...) -> c_int {
    let fmt = unsafe { CStr::from_ptr(fmt) };

    match crate::format::printf_to(
        unsafe { &mut **stdout.0.get() },
        fmt.to_bytes(),
        args.as_va_list(),
    ) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fprintf(stream: *mut File, fmt: *const c_char, mut args: ...) -> c_int {
    unsafe { vfprintf(stream, fmt, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vfprintf(
    stream: *mut File,
    fmt: *const c_char,
    args: core::ffi::VaList,
) -> c_int {
    let fmt = unsafe { CStr::from_ptr(fmt) };

    match crate::format::printf_to(unsafe { &mut *stream }, fmt.to_bytes(), args) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sprintf(s: *mut c_char, fmt: *const c_char, args: ...) -> c_int {
    unsafe { snprintf(s, strlen(s) + 1, fmt, args) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprintf(
    s: *mut c_char,
    n: usize,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    unsafe { vsnprintf(s, n, fmt, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vsnprintf(
    s: *mut c_char,
    n: usize,
    fmt: *const c_char,
    args: VaList,
) -> c_int {
    let fmt = unsafe { CStr::from_ptr(fmt) };
    let stream = unsafe { core::slice::from_raw_parts_mut(s as *mut u8, n) };
    match crate::format::printf_to(
        &mut BufWriter::new(&mut stream[..n - 1]),
        fmt.to_bytes(),
        args,
    ) {
        Ok(am) => {
            stream[am] = 0;
            am as c_int
        }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fscanf(stream: *mut File, fmt: *const c_char, mut args: ...) -> c_int {
    let fmt = unsafe { CStr::from_ptr(fmt) };
    let stream = unsafe { &mut *stream };
    try_errno!(
        crate::parse::scanf_from(stream, fmt.to_bytes(), args.as_va_list()),
        -1
    );
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sscanf(s: *const c_char, fmt: *const c_char, mut args: ...) -> c_int {
    let fmt = unsafe { CStr::from_ptr(fmt) };
    let stream = unsafe { core::slice::from_raw_parts(s as *const u8, strlen(s)) };
    let (_, matched) = try_errno!(
        crate::parse::scanf_from(
            &mut BufReader::new(stream),
            fmt.to_bytes(),
            args.as_va_list()
        ),
        -1
    );
    matched as c_int
}
