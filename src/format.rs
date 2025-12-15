use core::ffi::{CStr, VaListImpl, c_char, c_int, c_uint};
use core::fmt::Write;

use safa_api::errors::ErrorStatus;

use crate::errno::set_error;

#[derive(Debug)]
pub struct BufWriter<'a>(&'a mut [u8], usize);
impl<'a> BufWriter<'a> {
    pub const fn new(b: &'a mut [u8]) -> Self {
        Self(b, 0)
    }
}

pub trait CWriter {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, ErrorStatus>;
    fn write_byte(&mut self, b: u8) -> Result<usize, ErrorStatus> {
        self.write_bytes(&[b])
    }
}

impl<'a> CWriter for BufWriter<'a> {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, ErrorStatus> {
        if self.0.len() <= self.1 {
            return Ok(bytes.len());
        }

        let to = &mut self.0[self.1..];
        let amount = to.len().min(bytes.len());
        to[..amount].copy_from_slice(&bytes[..amount]);
        self.1 += amount;

        Ok(amount)
    }
}

impl<'a> Write for BufWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        map_error(self.write_bytes(s.as_bytes()).map(|_| ()))
    }
}

struct CWriterWrapper<'a, T: CWriter>(&'a mut T, usize);

impl<'a, T: CWriter> CWriter for CWriterWrapper<'a, T> {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, ErrorStatus> {
        let am = self.0.write_bytes(bytes)?;
        self.1 += am;
        Ok(am)
    }
}

impl<'a, T: CWriter> Write for CWriterWrapper<'a, T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        map_error(self.write_bytes(s.as_bytes()).map(|_| ()))
    }
}

fn until_fmt<'a, W: CWriter>(
    writer: &mut W,
    fmt: &'a [u8],
) -> Result<(&'a [u8], usize), ErrorStatus> {
    let mut current = fmt;
    let mut wrote = 0;
    while !fmt.is_empty() && fmt[0] != b'%' {
        wrote += writer.write_byte(fmt[0])?;
        current = &fmt[1..];
    }
    Ok((current, wrote))
}

fn map_error<O>(maybe: Result<O, ErrorStatus>) -> Result<O, core::fmt::Error> {
    maybe.map_err(|e| {
        set_error(e);
        core::fmt::Error
    })
}

macro_rules! try_fmt {
    ($expr: expr) => {{ map_error($expr)? }};
}

pub fn printf_to<W: CWriter>(
    writer: &mut W,
    fmt: &[u8],
    var_args: &mut VaListImpl,
) -> Result<usize, core::fmt::Error> {
    let mut current = fmt;
    let mut writer = CWriterWrapper(writer, 0);

    while !current.is_empty() {
        let (advanced, _) = try_fmt!(until_fmt(&mut writer, fmt));
        current = advanced;

        if current.len() == 0 {
            break;
        }
        current = &current[1..];
        match current[0] {
            b'%' => _ = try_fmt!(writer.write_byte(b'%')),
            b'c' => _ = write!(writer, "{}", unsafe { var_args.arg::<u32>() as u8 as char })?,
            b'z' => {
                let usize = unsafe { var_args.arg::<usize>() };
                match current[1] {
                    b'd' => {
                        let i = usize.cast_signed();
                        write!(writer, "{}", i)?;
                        current = &current[1..];
                    }
                    c => {
                        _ = write!(writer, "{}", usize);
                        if c == b'u' {
                            current = &current[1..];
                        }
                    }
                }
            }
            b's' => {
                let cstr_ptr = unsafe { var_args.arg::<*const c_char>() };
                let cstr = unsafe { CStr::from_ptr(cstr_ptr) };
                _ = try_fmt!(writer.write_bytes(cstr.to_bytes()));
            }
            b'x' => {
                let uint = unsafe { var_args.arg::<c_uint>() };
                _ = write!(writer, "{:x}", uint);
            }
            b'o' => {
                let uint = unsafe { var_args.arg::<c_uint>() };
                _ = write!(writer, "{:o}", uint);
            }
            b'X' => {
                let uint = unsafe { var_args.arg::<c_uint>() };
                _ = write!(writer, "{:X}", uint);
            }
            b'p' => {
                let ptr = unsafe { var_args.arg::<*const ()>() };
                _ = write!(writer, "{:?}", ptr);
            }
            b'u' => {
                let uint = unsafe { var_args.arg::<c_uint>() };
                _ = write!(writer, "{}", uint);
            }
            b'i' | b'd' => {
                let int = unsafe { var_args.arg::<c_int>() };
                _ = write!(writer, "{}", int);
            }
            _ => {}
        }
    }
    Ok(writer.1)
}
