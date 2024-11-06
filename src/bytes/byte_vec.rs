extern crate alloc;
use core::ops::{Deref, DerefMut};

use alloc::vec::Vec;

use crate::bytes::ByteSlice;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteVec(Vec<u8>);

impl ByteVec {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self(vec)
    }

    #[inline]
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }

    #[inline]
    pub fn from_bytes(bytes: &ByteSlice) -> Self {
        Self(bytes.to_vec())
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[inline]
    pub fn as_bytes(&self) -> &ByteSlice {
        ByteSlice::from_slice(self.as_slice())
    }
}

impl Deref for ByteVec {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for ByteVec {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}
