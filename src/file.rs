use core::{fmt::Write, mem::MaybeUninit};

use alloc::{boxed::Box, vec::Vec};
use safa_api::{
    abi::fs::OpenOptions,
    errors::ErrorStatus,
    syscalls::{self, fs, io, resources, types::Ri},
};

use crate::{errno::set_error, format::CWriter, parse::CReader};

const INITIAL_BUFFERING_LEN: usize = 1024;

#[derive(Debug, Clone, Copy)]
pub enum BufferingOption {
    None = 0,
    Buffered = 1,
    LineBuffered = 2,
}

impl BufferingOption {
    pub const fn from_u8(u8: u8) -> Option<Self> {
        Some(match u8 {
            0 => Self::None,
            1 => Self::Buffered,
            2 => Self::LineBuffered,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BufferedIO {
    LineBuffered {
        stdin_line: Vec<u8>,
        stdout_line: Vec<u8>,
    },
    SizeBuffered {
        stdin: Box<[u8]>,
        stdin_pos: usize,
        stdout: Box<[u8]>,
        stdout_pos: usize,
    },
    None,
}

impl BufferedIO {
    fn flush(&mut self, to: &mut FileUnbuffered) -> Result<usize, ErrorStatus> {
        match self {
            Self::None => Ok(0),
            Self::LineBuffered { stdout_line, .. } => {
                let r = to.write_unbuffered(&*stdout_line)?;
                stdout_line.clear();
                Ok(r)
            }
            Self::SizeBuffered {
                stdout, stdout_pos, ..
            } => {
                let r = to.write_unbuffered(&stdout[..*stdout_pos])?;
                *stdout_pos = 0;
                Ok(r)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SeekPosition {
    Current(isize),
    End(usize),
    Start(usize),
}

#[derive(Debug, PartialEq)]
struct FileUnbuffered {
    resource: Ri,
    offset: isize,
    eof: bool,
}

impl FileUnbuffered {
    /// Writes `bytes` to file at the current position.
    fn write_unbuffered(&mut self, bytes: &[u8]) -> Result<usize, ErrorStatus> {
        let results = io::write(self.resource, self.offset, bytes)?;
        if self.offset >= 0 || self.offset < -1 {
            self.offset += results as isize;
        }
        Ok(results)
    }

    /// Reads into `buf` from the file at the current position.
    fn read_unbuffered(&mut self, buf: &mut [u8]) -> Result<usize, ErrorStatus> {
        let results = io::read(self.resource, self.offset, buf)?;
        if self.offset >= 0 || self.offset < -1 {
            self.offset += results as isize;
        }

        self.eof = results == 0;
        Ok(results)
    }

    pub fn sync(&self) -> Result<(), ErrorStatus> {
        syscalls::io::sync(self.resource)
    }

    pub fn offset(&self) -> usize {
        if self.offset >= 0 {
            self.offset as usize
        } else {
            let size = self.size();
            size.saturating_add_signed(self.offset + 1)
        }
    }

    /// Returns the size of the file in bytes
    pub fn size(&self) -> usize {
        syscalls::io::fsize(self.resource).expect("fsize should never fail")
    }

    /// Changes the position at which the file reads and writes.
    pub fn seek(&mut self, wrench: SeekPosition) {
        match wrench {
            SeekPosition::Start(s) => self.offset = s as isize,
            SeekPosition::End(e) => self.offset = -((e + 1) as isize),
            SeekPosition::Current(c) => {
                if self.offset >= 0 || self.offset < -1 {
                    self.offset = self.offset.saturating_add(c);
                }
            }
        }
    }
}

impl CReader for FileUnbuffered {
    #[inline(always)]
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ErrorStatus> {
        self.read_unbuffered(buf)
    }
}

#[derive(Debug, PartialEq)]
pub struct File {
    inner: FileUnbuffered,
    buffering: BufferedIO,
}

impl File {
    pub fn from_res(res: Ri, option: BufferingOption, from_end: bool) -> Self {
        let mut this = Self {
            inner: FileUnbuffered {
                resource: res,
                offset: if from_end { -1 } else { 0 },
                eof: false,
            },
            buffering: BufferedIO::None,
        };
        this.set_buffering(option, 0);
        this
    }

    pub fn open_diriter(&self) -> Result<Ri, ErrorStatus> {
        syscalls::io::diriter_open(self.inner.resource)
    }

    pub fn open(path: &str, options: OpenOptions) -> Result<Self, ErrorStatus> {
        let ri = fs::open(path, options)?;
        Ok(Self {
            inner: FileUnbuffered {
                resource: ri,
                offset: 0,
                eof: false,
            },
            buffering: BufferedIO::None,
        })
    }

    pub fn close(mut self) -> Result<(), ErrorStatus> {
        unsafe { self.close_ref() }
    }

    pub unsafe fn close_ref(&mut self) -> Result<(), ErrorStatus> {
        _ = self.flush();
        resources::destroy_resource(self.inner.resource)
    }

    /// Writes `bytes` to file,
    ///
    /// writes may be buffer until a call to [`Self::flush`].
    pub fn write(&mut self, bytes: &[u8]) -> Result<usize, ErrorStatus> {
        let mut len = 0;

        match self.buffering {
            BufferedIO::LineBuffered {
                stdout_line: ref mut buf,
                ..
            } => {
                for c in bytes {
                    len += 1;
                    buf.push(*c);

                    if *c == b'\n' {
                        self.inner.write_unbuffered(&*buf)?;
                        buf.clear();
                    }
                }
            }

            BufferedIO::SizeBuffered {
                stdout: ref mut buf,
                ref mut stdout_pos,
                ..
            } => {
                for c in bytes {
                    len += 1;
                    buf[*stdout_pos] = *c;
                    *stdout_pos += 1;

                    if *stdout_pos >= buf.len() {
                        self.inner.write_unbuffered(&*buf)?;
                        *stdout_pos = 0;
                    }
                }
            }
            BufferedIO::None => return self.inner.write_unbuffered(bytes),
        }

        Ok(len)
    }

    /// Reads `bytes` from file,
    ///
    /// read may buffer, reading more than requested.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, ErrorStatus> {
        match &mut self.buffering {
            BufferedIO::None => return self.inner.read_unbuffered(buf),
            BufferedIO::LineBuffered {
                stdin_line: line, ..
            } => {
                if line.len() == 0 {
                    let am = self
                        .inner
                        .read_bytes_until_or_eof_alloc(line, usize::MAX, |c| c == b'\n')?;
                    if am == 0 {
                        return Ok(0);
                    }

                    return self.read(buf);
                } else {
                    let amount = buf.len().min(line.len());
                    buf[..amount].copy_from_slice(&line[..amount]);
                    return Ok(amount);
                }
            }
            BufferedIO::SizeBuffered {
                stdin, stdin_pos, ..
            } => {
                if *stdin_pos != 0 {
                    let amount = buf.len().min(*stdin_pos);
                    buf[..amount].copy_from_slice(&stdin[..amount]);
                    Ok(amount)
                } else {
                    let unfilled = &mut stdin[*stdin_pos..];
                    let am = self.inner.read_unbuffered(unfilled)?;
                    if am == 0 {
                        return Ok(0);
                    }

                    return self.read(buf);
                }
            }
        }
    }

    /// Changes the position at which the file reads and writes.
    pub fn seek(&mut self, wrench: SeekPosition) {
        self.inner.seek(wrench)
    }

    /// Returns the size of the file in bytes
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    pub fn offset(&self) -> usize {
        self.inner.offset()
    }

    pub fn flush(&mut self) -> Result<(), ErrorStatus> {
        let r = self.buffering.flush(&mut self.inner);
        let o_r = self.inner.sync();

        if r.is_err() {
            return r.map(|_| ());
        } else {
            return o_r;
        }
    }

    pub fn is_eof(&self) -> bool {
        self.inner.eof
    }

    /// Changes how this file is buffered.
    pub fn set_buffering(&mut self, option: BufferingOption, size: usize) {
        use alloc::vec;

        _ = self.flush();
        let size = size.max(INITIAL_BUFFERING_LEN);

        let io = match option {
            BufferingOption::None => BufferedIO::None,
            BufferingOption::LineBuffered => BufferedIO::LineBuffered {
                stdin_line: Vec::with_capacity(size),
                stdout_line: Vec::with_capacity(size),
            },
            BufferingOption::Buffered => BufferedIO::SizeBuffered {
                stdin: unsafe {
                    vec![MaybeUninit::uninit(); size]
                        .into_boxed_slice()
                        .assume_init()
                },
                stdin_pos: 0,
                stdout: unsafe {
                    vec![MaybeUninit::uninit(); size]
                        .into_boxed_slice()
                        .assume_init()
                },
                stdout_pos: 0,
            },
        };

        self.buffering = io;
    }
}

impl CReader for File {
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ErrorStatus> {
        self.read(buf)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { self.close_ref().expect("Failed to close File") }
    }
}

impl Write for File {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes())
            .map_err(|e| {
                set_error(e);
                core::fmt::Error
            })
            .map(|_| ())
    }
}

impl CWriter for File {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, ErrorStatus> {
        self.write(bytes)
    }
}
