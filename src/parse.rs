use core::ffi::{c_char, c_int, c_uint};

use alloc::vec::Vec;
use safa_api::errors::ErrorStatus;

#[derive(Debug)]
pub struct BufReader<'a>(&'a [u8], usize);
impl<'a> BufReader<'a> {
    pub const fn new(b: &'a [u8]) -> Self {
        Self(b, 0)
    }
}

pub trait CReader {
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ErrorStatus>;
    fn read_byte(&mut self) -> Result<Option<u8>, ErrorStatus> {
        let mut buf = [0u8];
        let r = self.read_bytes(&mut buf)?;
        if r == 0 {
            return Ok(None);
        } else {
            return Ok(Some(buf[0]));
        }
    }

    fn read_bytes_until_or_eof<F: FnMut(u8) -> bool>(
        &mut self,
        buf: &mut [u8],
        mut until: F,
    ) -> Result<usize, ErrorStatus> {
        let mut read = 0;

        while read < buf.len() {
            let c = self.read_byte()?;
            match c {
                Some(c) => {
                    buf[read] = c;
                    read += 1;
                    if until(c) {
                        break;
                    }
                }
                None => break,
            }
        }

        Ok(read)
    }

    fn read_bytes_until_or_eof_alloc<F: FnMut(u8) -> bool>(
        &mut self,
        buf: &mut Vec<u8>,
        max: usize,
        mut until: F,
    ) -> Result<usize, ErrorStatus> {
        let mut read = 0;

        while read < max {
            let c = self.read_byte()?;
            match c {
                Some(c) => {
                    buf.push(c);
                    read += 1;
                    if until(c) {
                        break;
                    }
                }
                None => break,
            }
        }

        Ok(read)
    }
}

impl<'a> CReader for BufReader<'a> {
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ErrorStatus> {
        if self.0.len() <= self.1 {
            return Ok(buf.len());
        }

        let to = &self.0[self.1..];
        let amount = to.len().min(buf.len());
        buf[..amount].copy_from_slice(&to[..amount]);
        self.1 += amount;

        Ok(amount)
    }
}

struct CReaderWrapper<'a, T: CReader>(&'a mut T, usize);

impl<'a, T: CReader> CReader for CReaderWrapper<'a, T> {
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ErrorStatus> {
        let am = self.0.read_bytes(buf)?;
        self.1 += am;
        Ok(am)
    }
}

pub fn scanf_from<R: CReader>(
    reader: &mut R,
    fmt: &[u8],
    mut var_args: core::ffi::VaList,
) -> Result<(usize, usize), ErrorStatus> {
    let mut fmt_iter = fmt.iter().peekable();
    let mut reader = CReaderWrapper(reader, 0);
    let mut matched = 0;

    macro_rules! parse_int {
        ($f: expr, $ty: ty, $radix:literal) => {
            let mut buf = [0u8; 64];
            let read = reader.read_bytes_until_or_eof(&mut buf, $f)?;
            let num_raw = &buf[..read];

            let num_str = unsafe { str::from_utf8_unchecked(&num_raw) };
            let num = <$ty>::from_str_radix(num_str, $radix).expect("Failed to parse an integer");
            unsafe {
                var_args.arg::<*mut $ty>().write(num as $ty);
            }

            matched += 1;
        };
    }

    while let Some(c) = fmt_iter.next() {
        if *c == b'%' {
            let Some(spec) = fmt_iter.next() else {
                break;
            };

            match spec {
                b'd' => {
                    let mut start = true;
                    parse_int!(
                        |c| {
                            let cond = !(c.is_ascii_digit() || (c == b'-' && start));
                            start = false;
                            cond
                        },
                        c_int,
                        10
                    );
                }
                b'u' => {
                    parse_int!(|c| !c.is_ascii_digit(), c_uint, 10);
                }
                b'x' | b'X' => {
                    let mut pos = 0;
                    parse_int!(
                        |c| {
                            let cond = !match pos {
                                0 => c == b'0',
                                1 => c == b'x',
                                _ => c.is_ascii_hexdigit(),
                            };
                            pos += 1;
                            cond
                        },
                        c_uint,
                        16
                    );
                }
                b's' => {
                    let next_char = fmt_iter.peek().copied().unwrap_or(&b'\0');
                    let ptr = unsafe { var_args.arg::<*mut c_char>() };
                    let mut pos = 0;
                    while let Some(c) = reader.read_byte()? {
                        if c == *next_char {
                            break;
                        }

                        unsafe { ptr.add(pos).write(c as c_char) };
                        pos += 1;
                    }
                    unsafe { ptr.add(pos).write(b'\0' as c_char) };
                    matched += 1;
                }
                _ => {}
            }
        } else {
            let r = reader.read_byte()?;
            if r != Some(*c) {
                break;
            }
        }
    }

    Ok((reader.1, matched))
}
