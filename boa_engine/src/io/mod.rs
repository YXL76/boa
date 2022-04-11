// #[cfg(not(no_std))]
// pub use io::{Read};
//
// #[cfg(no_std)]

use core::slice;

mod error;
mod impls;
mod std_error;

pub use error::{Error, ErrorKind, Result};
pub use std_error::Error as StdError;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn bytes(self) -> Bytes<Self>
    where
        Self: Sized,
    {
        Bytes { inner: self }
    }
}

#[derive(Debug)]
pub struct Bytes<R> {
    inner: R,
}

impl<R: Read> Iterator for Bytes<R> {
    type Item = Result<u8>;

    fn next(&mut self) -> Option<Result<u8>> {
        let mut byte = 0;
        loop {
            return match self.inner.read(slice::from_mut(&mut byte)) {
                Ok(0) => None,
                Ok(..) => Some(Ok(byte)),
                // Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => Some(Err(e)),
            };
        }
    }
}
