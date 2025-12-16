use core::ffi::{VaArgSafe, c_char, c_int, c_long, c_longlong, c_short, c_uint};
use core::ffi::{c_uchar, c_ulong, c_ulonglong, c_ushort};
use core::fmt::Write;

use safa_api::errors::ErrorStatus;

use crate::errno::set_error;
use crate::string::strlen;

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

fn map_error<O>(maybe: Result<O, ErrorStatus>) -> Result<O, core::fmt::Error> {
    maybe.map_err(|e| {
        set_error(e);
        core::fmt::Error
    })
}

macro_rules! try_fmt {
    ($expr: expr) => {{ map_error($expr)? }};
}

struct CPrinter<'a, 'fmt, 'b, 'f, W: CWriter> {
    writer: &'a mut W,
    wrote: usize,
    curr_index: usize,
    fmt: &'fmt [u8],
    var_args: core::ffi::VaList<'b, 'f>,
}

impl<'a, 'fmt, 'b, 'f, T: CWriter> CWriter for CPrinter<'a, 'fmt, 'b, 'f, T> {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, ErrorStatus> {
        let am = self.writer.write_bytes(bytes)?;
        self.wrote += am;
        Ok(am)
    }
}

impl<'a, 'fmt, 'b, 'f, T: CWriter> Write for CPrinter<'a, 'fmt, 'b, 'f, T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        map_error(self.write_bytes(s.as_bytes()).map(|_| ()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LengthModifier {
    None,
    /// hh
    Char,
    /// h
    Short,
    /// l
    Long,
    /// ll
    LongLong,
    /// L
    LongDouble,
    /// z
    SizeT,
    /// Z
    PtrDiffT,
    /// j
    MaxT,
}

enum Kind {
    Octal,
    Hex,
    BigHex,
    Normal,
}

macro_rules! match_arg {
    ($self:ident, $e: expr, { $($matched_e:pat => ($t0:ty, $t1:ty) $(as ($as0:ty, $as1:ty))?,)* }, $unsigned: expr, $precision: expr, $fmt_spec_no_prec:expr, $fmt_spec_prec:expr) => {
        match $e {
            $($matched_e => {
                if !$unsigned {
                    let arg = unsafe { $self.arg::<$t0>() $(as $as0)? };
                    if let Some(prec) = $precision {
                    write!($self, $fmt_spec_prec, arg, prec = prec)?;
                    } else {
                        write!($self, $fmt_spec_no_prec, arg)?;
                    }
                } else {
                    let arg = unsafe { $self.arg::<$t1>() $(as $as1)? };
                    if let Some(prec) = $precision {
                        write!($self, $fmt_spec_prec, arg, prec = prec)?;
                    } else {
                        write!($self, $fmt_spec_no_prec, arg)?;
                    }
                }
            }),*
        }
    };

}

impl<'a, 'fmt, 'b, 'f, W: CWriter> CPrinter<'a, 'fmt, 'b, 'f, W> {
    fn new(writer: &'a mut W, fmt: &'fmt [u8], var_args: core::ffi::VaList<'b, 'f>) -> Self {
        Self {
            writer,
            wrote: 0,
            curr_index: 0,
            fmt,
            var_args,
        }
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        self.fmt.get(self.curr_index).copied()
    }

    #[inline]
    pub fn next(&mut self) -> Option<u8> {
        let byte = self.peek();
        if byte.is_some() {
            self.curr_index += 1;
        }
        byte
    }

    unsafe fn arg<T: VaArgSafe>(&mut self) -> T {
        unsafe { self.var_args.arg() }
    }

    fn next_int(
        &mut self,
        unsigned: bool,
        kind: Kind,
        precision: Option<usize>,
        length: LengthModifier,
    ) -> core::fmt::Result {
        if length == LengthModifier::LongDouble {
            let f = unsafe { self.arg::<f64>() };

            return if let Some(prec) = precision {
                write!(self, "{:.prec$}", f, prec = prec)
            } else {
                write!(self, "{}", f)
            };
        }

        macro_rules! call_with {
            ($fmt_no_prec:literal, $fmt_prec:literal) => {
                match_arg!(
                    self,
                    length,
                    {
                    LengthModifier::None => (c_int, c_uint),
                    LengthModifier::Char => (c_int, c_int) as (c_char, c_uchar),
                    LengthModifier::Short => (c_int, c_int) as (c_short, c_ushort),
                    LengthModifier::Long => (c_long, c_ulong),
                    LengthModifier::LongLong => (c_longlong, c_ulonglong),
                    LengthModifier::LongDouble => (usize, usize),
                    LengthModifier::SizeT => (usize, usize),
                    LengthModifier::PtrDiffT => (isize, isize),
                    LengthModifier::MaxT => (isize, usize),
                    },
                    unsigned,
                    precision,
                    $fmt_no_prec,
                    $fmt_prec
                )
            };
        }

        match kind {
            Kind::Normal => call_with!("{}", "{:0prec$}"),
            Kind::Octal => call_with!("{:o}", "{:0prec$o}"),
            Kind::Hex => call_with!("{:x}", "{:0prec$x}"),
            Kind::BigHex => call_with!("{:X}", "{:0prec$X}"),
        };
        Ok(())
    }

    fn try_make_length(&mut self) -> Option<LengthModifier> {
        let Some(maybe_length) = self.peek() else {
            return None;
        };

        match maybe_length {
            b'h' => {
                self.next();

                Some(if self.peek() == Some(b'h') {
                    self.next();
                    LengthModifier::Char
                } else {
                    LengthModifier::Short
                })
            }
            b'l' => {
                self.next();

                Some(if self.peek() == Some(b'l') {
                    self.next();
                    LengthModifier::LongLong
                } else {
                    LengthModifier::Long
                })
            }
            b'j' => {
                self.next();
                Some(LengthModifier::MaxT)
            }
            b'z' => {
                self.next();
                Some(LengthModifier::SizeT)
            }
            b't' => {
                self.next();
                Some(LengthModifier::PtrDiffT)
            }
            b'L' => {
                self.next();
                Some(LengthModifier::LongDouble)
            }
            _ => None,
        }
    }
    fn write_next_fmt(
        &mut self,
        length: Option<LengthModifier>,
        precision: Option<usize>,
    ) -> core::fmt::Result {
        let Some(spec) = self.next() else {
            return Ok(());
        };

        match spec {
            b'c' => {
                let c = unsafe { self.arg::<c_int>() } as c_char;
                try_fmt!(self.write_byte(c as u8));
                Ok(())
            }
            b'd' | b'i' => self.next_int(
                false,
                Kind::Normal,
                precision,
                length.unwrap_or(LengthModifier::None),
            ),
            b'u' => self.next_int(
                true,
                Kind::Normal,
                precision,
                length.unwrap_or(LengthModifier::None),
            ),
            b'o' => self.next_int(
                true,
                Kind::Octal,
                precision,
                length.unwrap_or(LengthModifier::None),
            ),
            b'x' => self.next_int(
                true,
                Kind::Hex,
                precision,
                length.unwrap_or(LengthModifier::None),
            ),
            b'p' => {
                let ptr = unsafe { self.arg::<*const ()>() };
                write!(self, "{ptr:?}")
            }
            b'X' => self.next_int(
                true,
                Kind::BigHex,
                precision,
                length.unwrap_or(LengthModifier::None),
            ),
            b's' => {
                let ptr = unsafe { self.arg::<*const c_char>() };
                let len = if let Some(prec) = precision {
                    prec
                } else {
                    unsafe { strlen(ptr as *const _) }
                };

                let bytes = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
                try_fmt!(self.write_bytes(bytes));
                Ok(())
            }
            b => {
                if let Some(length) = length {
                    self.next_int(false, Kind::Normal, precision, length)?;
                    try_fmt!(self.write_byte(b));
                }

                Ok(())
            }
        }
    }

    fn write_all(mut self) -> Result<usize, core::fmt::Error> {
        while let Some(byte) = self.next() {
            if byte == b'%' {
                let Some(maybe_spec) = self.peek() else {
                    break;
                };

                match maybe_spec {
                    b'%' => {
                        self.next();
                        try_fmt!(self.write_byte(b'%'));
                    }
                    b'.' => {
                        _ = self.next();
                        let Some(maybe_precision) = self.next() else {
                            break;
                        };

                        let precision = match maybe_precision {
                            b'*' => unsafe { self.arg::<c_uint>() as usize },
                            p @ b'0'..b'9' => (p - b'0') as usize,
                            _ => continue,
                        };

                        let length = self.try_make_length();
                        self.write_next_fmt(length, Some(precision))?;
                    }
                    _ => {
                        let length = self.try_make_length();
                        self.write_next_fmt(length, None)?;
                    }
                }
            } else {
                try_fmt!(self.write_byte(byte));
            }
        }
        Ok(self.wrote)
    }
}

pub fn printf_to<'a, 'fmt, 'b, 'f, W: CWriter>(
    writer: &'a mut W,
    fmt: &'fmt [u8],
    var_args: core::ffi::VaList<'b, 'f>,
) -> Result<usize, core::fmt::Error> {
    let printer = CPrinter::new(writer, fmt, var_args);
    printer.write_all()
}
