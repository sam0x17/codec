extern crate alloc;
use core::{
    ops::{Deref, DerefMut},
    slice::SliceIndex,
};

use alloc::vec::Vec;

#[cfg(test)]
use alloc::vec;

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

impl<const N: usize> From<&[u8; N]> for ByteVec {
    #[inline]
    fn from(array: &[u8; N]) -> Self {
        Self::from_slice(array)
    }
}

impl<const N: usize> From<&mut [u8; N]> for ByteVec {
    #[inline]
    fn from(array: &mut [u8; N]) -> Self {
        Self::from_slice(array)
    }
}

#[test]
fn test_new() {
    let byte_vec = ByteVec::new();
    assert!(byte_vec.is_empty());
}

#[test]
fn test_with_capacity() {
    let byte_vec = ByteVec::with_capacity(10);
    assert_eq!(byte_vec.as_vec().capacity(), 10);
}

#[test]
fn test_from_vec() {
    let vec = vec![1, 2, 3];
    let byte_vec = ByteVec::from_vec(vec.clone());
    assert_eq!(byte_vec.as_slice(), vec.as_slice());
}

#[test]
fn test_from_slice() {
    let slice = &[1, 2, 3];
    let byte_vec = ByteVec::from_slice(slice);
    assert_eq!(byte_vec.as_slice(), slice);
}

#[test]
fn test_from_bytes() {
    let byte_slice = ByteSlice::from_slice(&[1, 2, 3]);
    let byte_vec = ByteVec::from_bytes(&byte_slice);
    assert_eq!(byte_vec.as_slice(), byte_slice.as_slice());
}

#[test]
fn test_as_slice() {
    let vec = vec![1, 2, 3];
    let byte_vec = ByteVec::from_vec(vec.clone());
    assert_eq!(byte_vec.as_slice(), vec.as_slice());
}

#[test]
fn test_as_bytes() {
    let vec = vec![1, 2, 3];
    let byte_vec = ByteVec::from_vec(vec.clone());
    assert_eq!(byte_vec.as_bytes().as_slice(), vec.as_slice());
}

#[test]
fn test_as_vec() {
    let vec = vec![1, 2, 3];
    let byte_vec = ByteVec::from_vec(vec.clone());
    assert_eq!(byte_vec.as_vec(), &vec);
}

#[test]
fn test_deref() {
    let vec = vec![1, 2, 3];
    let byte_vec = ByteVec::from_vec(vec.clone());
    assert_eq!(&*byte_vec, vec.as_slice());
}

#[test]
fn test_deref_mut() {
    let mut byte_vec = ByteVec::from_vec(vec![1, 2, 3]);
    byte_vec[0] = 4;
    assert_eq!(byte_vec.as_slice(), &[4, 2, 3]);
}

#[test]
fn test_len() {
    let byte_vec = ByteVec::from_vec(vec![1, 2, 3]);
    assert_eq!(byte_vec.len(), 3);
}

#[test]
fn test_is_empty() {
    let byte_vec = ByteVec::new();
    assert!(byte_vec.is_empty());
}

#[test]
fn test_split_at() {
    let byte_vec = ByteVec::from_vec(vec![1, 2, 3, 4]);
    let (left, right) = byte_vec.split_at(2);
    assert_eq!(left.as_slice(), &[1, 2]);
    assert_eq!(right.as_slice(), &[3, 4]);
}

#[test]
fn test_split_at_mut() {
    let mut byte_vec = ByteVec::from_vec(vec![1, 2, 3, 4]);
    let (left, right) = byte_vec.split_at_mut(2);
    left[0] = 5;
    right[0] = 6;
    assert_eq!(byte_vec.as_slice(), &[5, 2, 6, 4]);
}

#[test]
fn test_to_vec() {
    let byte_vec = ByteVec::from_vec(vec![1, 2, 3]);
    let cloned_vec = byte_vec.to_vec();
    assert_eq!(byte_vec, cloned_vec);
}
#[test]
fn test_get() {
    let byte_vec = ByteVec::from_vec(vec![1, 2, 3]);
    assert_eq!(byte_vec.get(1), Some(2));
    assert_eq!(byte_vec.get(3), None);
}

#[test]
fn test_get_mut() {
    let mut byte_vec = ByteVec::from_vec(vec![1, 2, 3]);
    if let Some(value) = byte_vec.get_mut(1) {
        *value = 4;
    }
    assert_eq!(byte_vec.as_slice(), &[1, 4, 3]);
}

#[test]
fn test_range() {
    let byte_vec = ByteVec::from_vec(vec![1, 2, 3, 4]);
    let range = byte_vec.range(1..3);
    assert_eq!(range.as_slice(), &[2, 3]);
}

#[test]
fn test_range_mut() {
    let mut byte_vec = ByteVec::from_vec(vec![1, 2, 3, 4]);
    let range = byte_vec.range_mut(1..3);
    range[0] = 5;
    assert_eq!(byte_vec.as_slice(), &[1, 5, 3, 4]);
}

#[test]
fn test_from_byte_slice() {
    let byte_slice = ByteSlice::from_slice(&[1, 2, 3]);
    let byte_vec = ByteVec::from(byte_slice);
    assert_eq!(byte_vec.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_from_vec_ref() {
    let vec = vec![1, 2, 3];
    let byte_vec = ByteVec::from(&vec);
    assert_eq!(byte_vec.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_from_slice_ref() {
    let slice = &[1u8, 2u8, 3u8];
    let byte_vec = ByteVec::from(slice);
    assert_eq!(byte_vec.as_slice(), slice);
}

#[test]
fn test_from_mut_vec() {
    let mut vec = vec![1, 2, 3];
    let byte_vec = ByteVec::from(&mut vec);
    assert_eq!(byte_vec.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_from_mut_slice() {
    let slice = &mut [1u8, 2u8, 3u8];
    let byte_vec = ByteVec::from(slice);
    assert_eq!(byte_vec.as_slice(), &[1, 2, 3]);
}
#[test]
fn test_with_capacity_zero() {
    let byte_vec = ByteVec::with_capacity(0);
    assert_eq!(byte_vec.as_vec().capacity(), 0);
    assert!(byte_vec.is_empty());
}

#[test]
fn test_from_empty_slice() {
    let slice: &[u8] = &[];
    let byte_vec = ByteVec::from_slice(slice);
    assert!(byte_vec.is_empty());
}

#[test]
fn test_split_at_bounds() {
    let byte_vec = ByteVec::from_vec(vec![1, 2, 3, 4]);
    let (left, right) = byte_vec.split_at(0);
    assert!(left.is_empty());
    assert_eq!(right.as_slice(), &[1, 2, 3, 4]);

    let (left, right) = byte_vec.split_at(4);
    assert_eq!(left.as_slice(), &[1, 2, 3, 4]);
    assert!(right.is_empty());
}
