extern crate alloc;
use core::{
    ops::{Deref, DerefMut},
    slice::SliceIndex,
};

use alloc::vec::Vec;

use super::Bytes;
use crate::bytes::ByteSlice;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteVec(Vec<u8>);

impl ByteVec {
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline]
    pub const fn from_vec(vec: Vec<u8>) -> Self {
        Self(vec)
    }

    #[inline]
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }

    #[inline]
    pub fn from_bytes(bytes: &ByteSlice) -> Self {
        Self(bytes.as_slice().to_vec())
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[inline]
    pub fn as_bytes(&self) -> &ByteSlice {
        ByteSlice::from_slice(self.as_slice())
    }

    #[inline]
    pub const fn as_vec(&self) -> &Vec<u8> {
        &self.0
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

impl Bytes for ByteVec {
    #[inline]
    fn as_bytes(&self) -> &ByteSlice {
        ByteSlice::from_slice(self.as_slice())
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<u8> {
        self.0.get(index).copied()
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        self.0.get_mut(index)
    }

    #[inline]
    fn range(&self, range: impl SliceIndex<[u8], Output = [u8]>) -> &ByteSlice {
        ByteSlice::from_slice(&self.0[range])
    }

    #[inline]
    fn range_mut(&mut self, range: impl SliceIndex<[u8], Output = [u8]>) -> &mut ByteSlice {
        ByteSlice::from_slice_mut(&mut self.0[range])
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    fn split_at(&self, mid: usize) -> (&ByteSlice, &ByteSlice) {
        let (left, right) = self.0.split_at(mid);
        (ByteSlice::from_slice(left), ByteSlice::from_slice(right))
    }

    #[inline]
    fn split_at_mut(&mut self, mid: usize) -> (&mut ByteSlice, &mut ByteSlice) {
        let (left, right) = self.0.split_at_mut(mid);
        (
            ByteSlice::from_slice_mut(left),
            ByteSlice::from_slice_mut(right),
        )
    }

    #[inline]
    fn to_vec(&self) -> ByteVec {
        self.clone()
    }
}

impl From<ByteVec> for Vec<u8> {
    #[inline]
    fn from(byte_vec: ByteVec) -> Self {
        byte_vec.0
    }
}

impl From<Vec<u8>> for ByteVec {
    #[inline]
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}

impl From<&ByteVec> for Vec<u8> {
    #[inline]
    fn from(byte_vec: &ByteVec) -> Self {
        byte_vec.0.clone()
    }
}

impl From<&Vec<u8>> for ByteVec {
    #[inline]
    fn from(vec: &Vec<u8>) -> Self {
        Self(vec.clone())
    }
}

impl From<&[u8]> for ByteVec {
    #[inline]
    fn from(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }
}

impl From<&ByteSlice> for ByteVec {
    #[inline]
    fn from(byte_slice: &ByteSlice) -> Self {
        Self(byte_slice.as_slice().to_vec())
    }
}

impl From<&mut ByteVec> for Vec<u8> {
    #[inline]
    fn from(byte_vec: &mut ByteVec) -> Self {
        byte_vec.0.clone()
    }
}

impl From<&mut Vec<u8>> for ByteVec {
    #[inline]
    fn from(vec: &mut Vec<u8>) -> Self {
        Self(vec.clone())
    }
}

impl From<&mut [u8]> for ByteVec {
    #[inline]
    fn from(slice: &mut [u8]) -> Self {
        Self(slice.to_vec())
    }
}

impl From<&mut ByteSlice> for ByteVec {
    #[inline]
    fn from(byte_slice: &mut ByteSlice) -> Self {
        Self(byte_slice.as_slice().to_vec())
    }
}

impl From<&mut ByteVec> for ByteVec {
    #[inline]
    fn from(byte_vec: &mut ByteVec) -> Self {
        byte_vec.clone()
    }
}

impl From<&mut ByteSlice> for Vec<u8> {
    #[inline]
    fn from(byte_slice: &mut ByteSlice) -> Self {
        byte_slice.as_slice().to_vec()
    }
}
