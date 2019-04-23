use super::{ErrorKind, Result};
use std::convert::AsRef;
use trackable::error::ErrorKindExt;

pub struct AlignedBuf {
    buf: Vec<u8>,
    offset: usize,
    len: usize,
}

impl AlignedBuf {
    pub fn new(len: usize) -> Result<AlignedBuf> {
        if len % 512 != 0 {
            return Err(ErrorKind::InvalidInput.cause(format!("{}", len)).into());
        }
        let v = vec![0; len + 511];
        let pos = v.as_ptr() as usize;
        let offset = if pos % 512 == 0 {
            0
        } else {
            ((pos + 511) / 512) * 512 - pos
        };
        Ok(AlignedBuf {
            buf: v,
            offset,
            len,
        })
    }

    /*
    pub fn as_ref(&self) -> &[u8] {
        &self.buf[self.offset..self.offset + self.len]
    }
    */

    pub fn as_mut_ref(&mut self) -> &mut [u8] {
        &mut self.buf[self.offset..self.offset + self.len]
    }
}

impl AsRef<[u8]> for AlignedBuf {
    fn as_ref(&self) -> &[u8] {
        &self.buf[self.offset..self.offset + self.len]
    }
}
