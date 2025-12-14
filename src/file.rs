use core::fmt::Write;

use safa_api::{
    abi::fs::OpenOptions,
    errors::ErrorStatus,
    syscalls::{fs, io, resources},
};

use crate::errno::set_error;

#[derive(Debug, Clone, Copy)]
pub enum SeekPosition {
    Current(usize),
    End(usize),
    Start(usize),
}

#[derive(Debug)]
pub struct File {
    resource: Ri,
    offset: isize,
}

impl File {
    pub fn open(path: &str, options: OpenOptions) -> Result<Self, ErrorStatus> {
        let ri = fs::open(path, options)?;
        Ok(Self {
            resource: ri,
            offset: 0,
        })
    }

    pub fn close(mut self) -> Result<Self, ErrorStatus> {
        unsafe { self.close_ref() }
    }

    pub unsafe fn close_ref(&mut self) -> Result<Self, ErrorStatus> {
        resources::destroy_resource(self.resource)
    }

    /// Writes `bytes` to file at the current position.
    pub fn write(&mut self, bytes: &[u8]) -> Result<usize, ErrorStatus> {
        let results = io::write(self.resource, self.offset, bytes)?;
        if self.offset >= 0 {
            self.offset += results;
        }
        Ok(results)
    }

    /// Reads into `buf` from the file at the current position.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, ErrorStatus> {
        let results = io::read(self.resource, self.offset, buf)?;
        if self.offset >= 0 {
            self.offset += results;
        }
        Ok(results)
    }

    /// Changes the position at which the file reads and writes.
    pub fn seek(&mut self, wrench: SeekPosition) {
        match wrench {
            SeekPosition::Start(s) => self.offset = s as isize,
            SeekPosition::End(e) => self.offset = -((e + 1) as isize),
            SeekPosition::Current(c) => {
                if self.offset >= 0 || self.offset < -1 {
                    self.offset = self.offset.saturating_add_unsigned(c);
                }
            }
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        resources::destroy_resource(self.resource).expect("Failed to close FILE")
    }
}

impl Write for File {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes()).map_err(|e| {
            set_error(e);
            core::fmt::Error
        })
    }
}
