#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::fmt::{Debug, Formatter};

use crate::byte_slice::ByteSlice;

pub enum ReadError {
    InsufficientData,
    #[cfg(feature = "std")]
    IoError(std::io::Error),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for ReadError {
    fn from(e: std::io::Error) -> Self {
        ReadError::IoError(e)
    }
}

impl Debug for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ReadError::InsufficientData => {
                write!(f, "there is not enough data to complete the requested read")
            }
            #[cfg(feature = "std")]
            ReadError::IoError(e) => write!(f, "{}", e),
        }
    }
}

pub enum WriteError {
    InsufficientSpace,
    #[cfg(feature = "std")]
    IoError(std::io::Error),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for WriteError {
    fn from(e: std::io::Error) -> Self {
        WriteError::IoError(e)
    }
}

impl Debug for WriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            WriteError::InsufficientSpace => {
                write!(
                    f,
                    "there is not enough space to complete the requested write"
                )
            }
            #[cfg(feature = "std")]
            WriteError::IoError(e) => write!(f, "{}", e),
        }
    }
}

pub trait Read {
    fn read<const N: usize>(&mut self) -> Result<[u8; N], ReadError>;
    fn read_dynamic(&mut self, n: usize) -> Result<Vec<u8>, ReadError>;
    fn read_to_end(&mut self) -> Vec<u8>;
}

impl Read for &[u8] {
    fn read<const N: usize>(&mut self) -> Result<[u8; N], ReadError> {
        let Some((ret, rest)) = self.split_first_chunk() else {
            return Err(ReadError::InsufficientData);
        };
        *self = rest;
        Ok(*ret)
    }

    fn read_dynamic(&mut self, n: usize) -> Result<Vec<u8>, ReadError> {
        if n > self.len() {
            return Err(ReadError::InsufficientData);
        }
        let (ret, rest) = self.split_at(n);
        *self = rest;
        Ok(ret.to_vec())
    }

    fn read_to_end(&mut self) -> Vec<u8> {
        let ret = self.to_vec();
        *self = &[];
        ret
    }
}

pub trait Write {
    fn write<'a>(&mut self, data: impl Into<&'a ByteSlice>) -> Result<(), WriteError>;
}

impl Write for &mut ByteSlice {
    fn write<'a>(&mut self, data: impl Into<&'a ByteSlice>) -> Result<(), WriteError> {
        let data = data.into();
        if self.len() < data.len() {
            return Err(WriteError::InsufficientSpace);
        }
        let (to_write, rest) = core::mem::take(self).split_at_mut(data.len());
        to_write.copy_from_slice(data);
        *self = ByteSlice::from_slice_mut(rest);
        Ok(())
    }
}

#[test]
fn test_read() {
    let buf = [1, 2, 3, 4, 5];
    let mut slice = &buf[..];
    let data: [u8; 2] = slice.read::<2>().unwrap();
    assert_eq!(data, [1, 2]);
    assert_eq!(slice, &[3, 4, 5]);
    let data: [u8; 3] = slice.read::<3>().unwrap();
    assert_eq!(data, [3, 4, 5]);
    assert_eq!(slice, &[]);
    let result = slice.read::<2>();
    assert!(result.is_err());
}

#[test]
fn test_read_dynamic() {
    let buf = [1, 2, 3, 4, 5];
    let mut slice = &buf[..];
    let data = slice.read_dynamic(2).unwrap();
    assert_eq!(&data, &[1, 2]);
    assert_eq!(slice, &[3, 4, 5]);
    let data = slice.read_dynamic(3).unwrap();
    assert_eq!(&data, &[3, 4, 5]);
    assert_eq!(slice, &[]);
    let result = slice.read_dynamic(2);
    assert!(result.is_err());
}

#[test]
fn test_read_to_end() {
    let buf = [1, 2, 3, 4, 5];
    let mut slice = &buf[..];
    let data = slice.read_to_end();
    assert_eq!(&data, &[1, 2, 3, 4, 5]);
    assert_eq!(slice, &[]);
}

#[test]
fn test_write() {
    let mut buf = [0; 5];
    let slice = &mut ByteSlice::from_slice_mut(&mut buf);
    slice.write(&[1, 2]).unwrap();
    slice.write(&[3, 4]).unwrap();
    let result = slice.write(&[6, 7, 8]);
    assert!(result.is_err());
    slice.write(&[5]).unwrap();
    assert_eq!(buf, [1, 2, 3, 4, 5]);
}
