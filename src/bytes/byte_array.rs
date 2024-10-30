use crate::bytes::ByteSlice;
use core::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ByteArray<const N: usize>([u8; N]);

impl<const N: usize> ByteArray<N> {
    #[inline]
    pub const fn new() -> Self {
        ByteArray([0; N])
    }

    #[inline]
    pub const fn from_slice(slice: &[u8; N]) -> Self {
        ByteArray(*slice)
    }

    #[inline]
    pub const fn as_slice(&self) -> &[u8; N] {
        &self.0
    }

    #[inline]
    pub const fn as_bytes(&self) -> &ByteSlice {
        &ByteSlice::from_slice(self.as_slice())
    }

    #[inline]
    pub const fn split_at(&self, mid: usize) -> (&ByteSlice, &ByteSlice) {
        let (left, right) = self.0.split_at(mid);
        (ByteSlice::from_slice(left), ByteSlice::from_slice(right))
    }

    #[inline]
    pub const fn split_at_checked(&self, mid: usize) -> Option<(&ByteSlice, &ByteSlice)> {
        match self.0.split_at_checked(mid) {
            Some((left, right)) => {
                Some((ByteSlice::from_slice(left), ByteSlice::from_slice(right)))
            }
            None => None,
        }
    }

    #[inline]
    pub const fn split_at_unchecked(&self, mid: usize) -> (&ByteSlice, &ByteSlice) {
        let (left, right) = unsafe { self.0.split_at_unchecked(mid) };
        (ByteSlice::from_slice(left), ByteSlice::from_slice(right))
    }

    #[inline]
    pub const fn is_ascii(&self) -> bool {
        self.0.is_ascii()
    }

    #[inline]
    pub const fn trim_ascii_start(&self) -> &ByteSlice {
        ByteSlice::from_slice(self.0.trim_ascii_start())
    }

    #[inline]
    pub const fn trim_ascii_end(&self) -> &ByteSlice {
        ByteSlice::from_slice(self.0.trim_ascii_end())
    }

    #[inline]
    pub const fn trim_ascii(&self) -> &ByteSlice {
        ByteSlice::from_slice(self.0.trim_ascii())
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub const fn first(&self) -> Option<&u8> {
        self.0.first()
    }

    #[inline]
    pub const fn last(&self) -> Option<&u8> {
        self.0.last()
    }

    #[inline]
    pub const fn split_first(&self) -> Option<(&u8, &ByteSlice)> {
        match self.0.split_first() {
            Some((first, rest)) => Some((first, ByteSlice::from_slice(rest))),
            None => None,
        }
    }

    #[inline]
    pub const fn split_last(&self) -> Option<(&u8, &ByteSlice)> {
        match self.0.split_last() {
            Some((last, rest)) => Some((last, ByteSlice::from_slice(rest))),
            None => None,
        }
    }

    #[inline]
    pub const fn first_chunk<const N2: usize>(&self) -> Option<&[u8; N2]> {
        self.0.first_chunk()
    }

    #[inline]
    pub const fn split_first_chunk<const N2: usize>(&self) -> Option<(&[u8; N2], &[u8])> {
        self.0.split_first_chunk()
    }

    #[inline]
    pub const fn split_last_chunk<const N2: usize>(&self) -> Option<(&[u8], &[u8; N2])> {
        self.0.split_last_chunk()
    }

    #[inline]
    pub const fn last_chunk<const N2: usize>(&self) -> Option<&[u8; N2]> {
        self.0.last_chunk()
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub const fn as_ptr_range(&self) -> core::ops::Range<*const u8> {
        self.0.as_ptr_range()
    }
}

impl<const N: usize> PartialEq<&ByteSlice> for ByteArray<N> {
    fn eq(&self, other: &&ByteSlice) -> bool {
        self.0 == other.as_slice()
    }
}

impl<const N: usize> PartialEq<ByteArray<N>> for &ByteSlice {
    fn eq(&self, other: &ByteArray<N>) -> bool {
        self.as_slice() == other.0
    }
}

impl<const N: usize> PartialEq<[u8; N]> for ByteArray<N> {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == *other
    }
}

impl<const N: usize> PartialEq<ByteArray<N>> for [u8; N] {
    fn eq(&self, other: &ByteArray<N>) -> bool {
        *self == other.0
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for ByteArray<N> {
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.0 == **other
    }
}

impl<const N: usize> PartialEq<ByteArray<N>> for &[u8; N] {
    fn eq(&self, other: &ByteArray<N>) -> bool {
        **self == other.0
    }
}

impl<const N: usize> PartialEq<[u8]> for ByteArray<N> {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl<const N: usize> PartialEq<ByteSlice> for ByteArray<N> {
    fn eq(&self, other: &ByteSlice) -> bool {
        self.0 == other.as_slice()
    }
}

impl<const N: usize> PartialEq<ByteArray<N>> for [u8] {
    fn eq(&self, other: &ByteArray<N>) -> bool {
        *self == other.0
    }
}

impl<const N: usize> PartialEq<&[u8]> for ByteArray<N> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0 == **other
    }
}

impl<const N: usize> PartialEq<ByteArray<N>> for &[u8] {
    fn eq(&self, other: &ByteArray<N>) -> bool {
        **self == other.0
    }
}

impl<const N: usize> From<[u8; N]> for ByteArray<N> {
    fn from(bytes: [u8; N]) -> Self {
        ByteArray(bytes)
    }
}

impl<const N: usize> From<ByteArray<N>> for [u8; N] {
    fn from(bytes: ByteArray<N>) -> Self {
        bytes.0
    }
}

impl<const N: usize> From<&[u8; N]> for ByteArray<N> {
    fn from(bytes: &[u8; N]) -> Self {
        ByteArray(*bytes)
    }
}

impl<'a, const N: usize> From<&'a ByteArray<N>> for &'a [u8; N] {
    fn from(bytes: &'a ByteArray<N>) -> Self {
        &bytes.0
    }
}

impl<const N: usize> Deref for ByteArray<N> {
    type Target = [u8; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for ByteArray<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test_bytes_sized_new() {
    let bytes = ByteArray::<4>::new();
    assert_eq!(*bytes, [0; 4]);
}

#[test]
fn test_bytes_sized_from_slice() {
    let slice = [1, 2, 3, 4];
    let bytes = ByteArray::from_slice(&slice);
    assert_eq!(*bytes, slice);
}

#[test]
fn test_bytes_sized_as_slice() {
    let slice = [1, 2, 3, 4];
    let bytes = ByteArray::from_slice(&slice);
    assert_eq!(bytes.as_slice(), &slice);
}

#[test]
fn test_bytes_sized_as_bytes() {
    let slice = [1, 2, 3, 4];
    let bytes = ByteArray::from_slice(&slice);
    assert_eq!(bytes.as_bytes(), ByteSlice::from_slice(&slice));
}

#[test]
fn test_bytes_sized_split_at() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4, 5]);
    let (left, right) = bytes.split_at(3);
    assert_eq!(left, ByteSlice::from_slice(&[1, 2, 3]));
    assert_eq!(right, ByteSlice::from_slice(&[4, 5]));
}

#[test]
fn test_bytes_sized_split_at_checked() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4, 5]);
    let (left, right) = bytes.split_at_checked(3).unwrap();
    assert_eq!(left, ByteSlice::from_slice(&[1, 2, 3]));
    assert_eq!(right, ByteSlice::from_slice(&[4, 5]));

    assert_eq!(bytes.split_at_checked(6), None);
}

#[test]
fn test_bytes_sized_split_at_unchecked() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4, 5]);
    let (left, right) = bytes.split_at_unchecked(2);
    assert_eq!(left, ByteSlice::from_slice(&[1, 2]));
    assert_eq!(right, ByteSlice::from_slice(&[3, 4, 5]));
}

#[test]
fn test_bytes_sized_is_ascii() {
    let ascii_bytes = ByteArray::from_slice(b"Hello");
    assert!(ascii_bytes.is_ascii());

    let non_ascii_bytes = ByteArray::from_slice(&[0xFF, 0x80, 0x7F, 0x40]);
    assert!(!non_ascii_bytes.is_ascii());
}

#[test]
fn test_bytes_sized_trim_ascii_start() {
    let bytes = ByteArray::from_slice(b"   Hello");
    let trimmed = bytes.trim_ascii_start();
    assert_eq!(trimmed, ByteSlice::from_slice(b"Hello"));
}

#[test]
fn test_bytes_sized_trim_ascii_end() {
    let bytes = ByteArray::from_slice(b"Hello   ");
    let trimmed = bytes.trim_ascii_end();
    assert_eq!(trimmed, ByteSlice::from_slice(b"Hello"));
}

#[test]
fn test_bytes_sized_trim_ascii() {
    let bytes = ByteArray::from_slice(b"   Hello   ");
    let trimmed = bytes.trim_ascii();
    assert_eq!(trimmed, ByteSlice::from_slice(b"Hello"));
}

#[test]
fn test_bytes_sized_is_empty() {
    let bytes = ByteArray::from_slice(&[]);
    assert!(bytes.is_empty());

    let non_empty_bytes = ByteArray::from_slice(&[1]);
    assert!(!non_empty_bytes.is_empty());
}

#[test]
fn test_bytes_sized_len() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    assert_eq!(bytes.len(), 4);
}

#[test]
fn test_bytes_sized_first() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    assert_eq!(bytes.first(), Some(&1));

    let empty_bytes = ByteArray::<0>::new();
    assert_eq!(empty_bytes.first(), None);
}

#[test]
fn test_bytes_sized_last() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    assert_eq!(bytes.last(), Some(&4));

    let empty_bytes = ByteArray::<0>::new();
    assert_eq!(empty_bytes.last(), None);
}

#[test]
fn test_bytes_sized_split_first() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    let (first, rest) = bytes.split_first().unwrap();
    assert_eq!(first, &1);
    assert_eq!(rest, ByteSlice::from_slice(&[2, 3, 4]));

    let empty_bytes = ByteArray::<0>::new();
    assert_eq!(empty_bytes.split_first(), None);
}

#[test]
fn test_bytes_sized_split_last() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    let (last, rest) = bytes.split_last().unwrap();
    assert_eq!(last, &4);
    assert_eq!(rest, ByteSlice::from_slice(&[1, 2, 3]));

    let empty_bytes = ByteArray::<0>::new();
    assert_eq!(empty_bytes.split_last(), None);
}

#[test]
fn test_bytes_sized_first_chunk() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(bytes.first_chunk::<2>(), Some(&[1, 2]));
    assert_eq!(bytes.first_chunk::<6>(), None);
}

#[test]
fn test_bytes_sized_last_chunk() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(bytes.last_chunk::<2>(), Some(&[4, 5]));
    assert_eq!(bytes.last_chunk::<6>(), None);
}

#[test]
fn test_bytes_sized_split_first_chunk() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4, 5]);
    let (chunk, rest) = bytes.split_first_chunk::<2>().unwrap();
    assert_eq!(chunk, &[1, 2]);
    assert_eq!(rest, &[3, 4, 5]);

    let bytes = ByteArray::from_slice(&[1, 2]);
    assert_eq!(bytes.split_first_chunk::<3>(), None);
}

#[test]
fn test_bytes_sized_split_last_chunk() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4, 5]);
    let (rest, chunk) = bytes.split_last_chunk::<2>().unwrap();
    assert_eq!(rest, &[1, 2, 3]);
    assert_eq!(chunk, &[4, 5]);

    let bytes = ByteArray::from_slice(&[1, 2]);
    assert_eq!(bytes.split_last_chunk::<3>(), None);
}

#[test]
fn test_bytes_sized_as_ptr() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    assert_eq!(bytes.as_ptr(), bytes.as_slice().as_ptr());
}

#[test]
fn test_bytes_sized_as_ptr_range() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    assert_eq!(bytes.as_ptr_range(), bytes.as_slice().as_ptr_range());
}

#[test]
fn test_bytes_sized_deref() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    assert_eq!(bytes[0], 1);
    assert_eq!(bytes[1], 2);
    assert_eq!(bytes[2], 3);
    assert_eq!(bytes[3], 4);
}

#[test]
fn test_bytes_sized_deref_mut() {
    let mut bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    bytes[2] = 99;
    assert_eq!(*bytes, [1, 2, 99, 4]);
}

#[test]
fn test_bytes_sized_equality() {
    let bytes1 = ByteArray::from_slice(&[1, 2, 3, 4]);
    let bytes2 = ByteArray::from_slice(&[1, 2, 3, 4]);
    let bytes3 = ByteArray::from_slice(&[1, 2, 3, 5]);

    assert_eq!(bytes1, bytes2);
    assert_ne!(bytes1, bytes3);
}

#[test]
fn test_bytes_sized_from_array() {
    let arr = [1, 2, 3, 4];
    let bytes: ByteArray<4> = arr.into();
    assert_eq!(*bytes, arr);
}

#[test]
fn test_bytes_sized_into_array() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    let arr: [u8; 4] = bytes.into();
    assert_eq!(arr, [1, 2, 3, 4]);
}

#[test]
fn test_bytes_sized_partial_eq_with_slice() {
    let bytes = ByteArray::from_slice(&[1, 2, 3, 4]);
    assert_eq!(bytes, &[1, 2, 3, 4]);
    assert_ne!(bytes, &[1, 2, 3, 5]);
}
